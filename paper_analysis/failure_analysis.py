"""Vanilla detection baseline and output-failure categories from raw logs."""
import json
import re
import zipfile
from collections import Counter
from common import LOGS, MODELS, MODEL_ORDER, NAMES, OUT, write_csv, write_table


def valid_answer(value):
    fields = ['is_vulnerable', 'cwe_type', 'vulnerable_functions', 'vulnerable_lines']
    return (isinstance(value, dict) and all(k in value for k in fields)
            and isinstance(value['is_vulnerable'], bool)
            and isinstance(value['cwe_type'], list)
            and all(isinstance(cwe, str) for cwe in value['cwe_type'])
            and all(isinstance(value[k], dict) and
                    all(isinstance(v, list) for v in value[k].values())
                    for k in fields[2:])
            and all(isinstance(line, int) for lines in value['vulnerable_lines'].values()
                    for line in lines))


def answer_in_text(sample):
    messages = [m for m in sample['messages'] if m['role'] == 'assistant']
    text = messages[-1]['content'] if messages else ''
    if isinstance(text, list):
        text = '\n'.join(p.get('text', '') or '' for p in text if isinstance(p, dict))
    decoder = json.JSONDecoder()
    for match in re.finditer(r'\{', text or ''):
        try:
            value, _ = decoder.raw_decode(text[match.start():])
        except ValueError:
            continue
        if valid_answer(value):
            return True
    return False


categories = ['answer_in_text', 'message_limit', 'invalid_output', 'no_final_answer']
results, details = {}, []
for path in sorted(LOGS.glob('*.eval')):
    with zipfile.ZipFile(path) as archive:
        header = json.loads(archive.read('header.json'))
        if header['eval']['task'] != 'mizan-vanilla':
            continue
        model = MODELS[header['eval']['model']]
        assert model not in results
        counts = Counter()
        samples = vulnerable = 0
        for member in archive.namelist():
            if not member.startswith('samples/'):
                continue
            sample = json.loads(archive.read(member))
            samples += 1
            vulnerable += bool(sample['metadata']['is_vulnerable'])
            score = sample['scores']['rustmizan_scorer']
            status = score['answer']
            if status == 'valid':
                continue
            assert status in ('no_file', 'invalid'), (sample['id'], status)
            if status == 'invalid':
                category = 'invalid_output'
            elif sample.get('limit'):
                assert sample['limit']['type'] == 'message', sample['limit']
                category = 'message_limit'
            elif answer_in_text(sample):
                category = 'answer_in_text'
            else:
                category = 'no_final_answer'
            counts[category] += 1
            counters = score['metadata']
            assert all(counters[k] == 0 for k in
                       ['binary_accuracy', 'success_at_1_function', 'success_at_1_line',
                        *[f'{task}_{kind}' for task in ('cwe', 'function', 'line')
                          for kind in ('tp', 'fp', 'fn')]])
            details.append([NAMES[model], sample['id'], category])
        results[model] = {'samples': samples, 'vulnerable': vulnerable,
                          'baseline_accuracy': 100 * vulnerable / samples,
                          'failures': {k: counts[k] for k in categories}}

assert set(results) == set(MODEL_ORDER)
write_csv('output_failures.csv', ['model', 'samples', *categories],
          [[NAMES[m], results[m]['samples'], *results[m]['failures'].values()]
           for m in MODEL_ORDER])
write_csv('output_failure_samples.csv', ['model', 'sample_id', 'category'], details)
rows = [[NAMES[m], *[f"{n} ({100*n/results[m]['samples']:.1f}\\%)"
                    for n in results[m]['failures'].values()]] for m in MODEL_ORDER]
write_table('output_failures.tex', ['Model', 'Answer in text', 'Message limit',
                                  'Invalid output', 'No final answer'], rows)
(OUT / 'output_failures.json').write_text(json.dumps(results, indent=2) + '\n')
print(json.dumps(results, indent=2))
