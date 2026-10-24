"""Reasoning effort comparison on matched vulnerable variants."""
import json
import zipfile
from collections import defaultdict
import numpy as np
from common import LOGS, OUT, MODEL_ORDER, NAMES, load_logs, load_configs, micro, write_csv

original = load_logs()
runs = {NAMES[m]: {sid: r for sid, r in original['vanilla', m].items()
                   if r['is_vulnerable_gt']} for m in MODEL_ORDER}
reference = runs[NAMES[MODEL_ORDER[0]]]
settings = {}
for config in load_configs():
    if config['split'] == 'vanilla':
        setting = {**config['plan_config'], **config['model_args']}
        settings[NAMES[config['model']]] = ('Disabled' if setting.get('reasoning_enabled') is False
                                           else setting['reasoning_effort'].title())
configs = {}
for effort in ('low', 'high'):
    with zipfile.ZipFile(LOGS / 'opus' / f'opus-{effort}.eval') as archive:
        header = json.loads(archive.read('header.json'))
        assert header['status'] == 'success'
        assert header['eval']['model'] == 'anthropic/claude-opus-5'
        assert header['plan']['config']['reasoning_effort'] == effort
        rows = {}
        for member in archive.namelist():
            if not (member.startswith('samples/') and member.endswith('.json')):
                continue
            sample = json.loads(archive.read(member))
            sid, metadata = sample['id'], sample['metadata']
            score = sample['scores']['rustmizan_scorer']
            row = dict(score['metadata'])
            assert sample['epoch'] == 1 and sid not in rows
            assert score['answer'] == 'valid' and row['is_vulnerable_gt']
            for field in ('vuln_id', 'crate_name', 'granularity', 'year', 'cwe_type'):
                assert metadata[field] == reference[sid][field]
            predicted = row['parsed_response']
            assert predicted == json.loads(row['artifacts']['results.json'])
            assert row['binary_accuracy'] == int(predicted['is_vulnerable'])
            for prefix, field in (('cwe', 'cwe_type'), ('function', 'vulnerable_functions'),
                                  ('line', 'vulnerable_lines')):
                elements = lambda x: {(f, v) for f, values in x.items() for v in values}
                p, a = ((set(predicted[field]), set(metadata[field])) if prefix == 'cwe'
                        else (elements(predicted[field]), elements(metadata[field])))
                assert (len(p & a), len(p - a), len(a - p)) == tuple(
                    row[f'{prefix}_{k}'] for k in ('tp', 'fp', 'fn'))
            for task in ('function', 'line'):
                assert row[f'success_at_1_{task}'] == int(row[f'{task}_tp'] > 0)
            row.update({k: metadata[k] for k in ('vuln_id', 'granularity')})
            rows[sid] = row
        assert set(rows) == set(reference) and len(rows) == 95
        assert header['results']['completed_samples'] == 95
        name = f'Claude Opus 5 ({effort})'
        settings[name] = effort.title()
        runs[name] = rows
        configs[name] = {'model': header['eval']['model'], 'effort': effort,
                         'config': header['eval']['config'],
                         'packages': header['eval'].get('packages'),
                         'revision': header['eval'].get('revision')}

results = {}
clusters = sorted({r['vuln_id'] for r in reference.values()})
assert len(clusters) == 42
weights = np.random.default_rng(20261004).multinomial(42, np.full(42, 1/42), size=200000)
bootstrap = {}
for name, rows in runs.items():
    values = list(rows.values())
    assert set(rows) == set(reference)
    result = {**{task + '_f1': micro(values, task)[2] for task in ('cwe', 'function', 'line')},
              **{task + '_success': 100 * sum(r['success_at_1_' + task] for r in values) / 95
                 for task in ('function', 'line')}}
    tensor = np.zeros((42, 3), dtype=np.int64)
    for row in values:
        tensor[clusters.index(row['vuln_id'])] += [row['line_' + k] for k in ('tp', 'fp', 'fn')]
    sums = weights @ tensor
    denominator = 2*sums[:, 0] + sums[:, 1] + sums[:, 2]
    draws = np.divide(200*sums[:, 0], denominator, out=np.full(denominator.shape, 100.),
                      where=denominator != 0)
    bootstrap[name] = draws
    result['line_f1_ci'] = np.quantile(draws, [.025, .975], method='linear').tolist()
    grouped = defaultdict(list)
    for row in values:
        grouped[row['vuln_id']].append(bool(row['success_at_1_line']))
    result['per_cve'] = {category: sum(predicate(v) for v in grouped.values())
                         for category, predicate in [('none', lambda v: not any(v)),
                                                      ('partial', lambda v: any(v) and not all(v)),
                                                      ('all', all)]}
    results[name] = result

columns = ['cwe_f1', 'function_f1', 'line_f1', 'function_success', 'line_success']
rows = [[name, settings[name], *[f'{result[k]:.1f}' for k in columns]]
        for name, result in results.items()]
write_csv('reasoning_effort.csv', ['model', 'reasoning', *columns], rows)
text = [r'\begin{tabular}{@{}llccccc@{}}', r'\toprule',
        r' & & \textbf{Classification} & \multicolumn{2}{c}{\textbf{Localization (F1)}} & \multicolumn{2}{c}{\textbf{Success@1}} \\',
        r'\cmidrule(lr){3-3} \cmidrule(lr){4-5} \cmidrule(lr){6-7}',
        r'\textbf{Model} & \textbf{Reasoning} & \textbf{CWE F1} & \textbf{Func.} & \textbf{Line} & \textbf{Func.} & \textbf{Line} \\',
        r'\midrule']
for name, setting, *values in rows:
    model = name.split(' (')[0]
    text.append(' & '.join([model, setting, *values]) + r' \\')
text += [r'\bottomrule', r'\end{tabular}']
(OUT / 'reasoning_effort.tex').write_text('\n'.join(text) + '\n')
difference = bootstrap['Claude Opus 5 (high)'] - bootstrap['Claude Opus 5 (low)']
output = {'results': results, 'configs': configs, 'reasoning_settings': settings, 'resamples': 200000, 'seed': 20261004,
          'high_minus_low_ci': np.quantile(difference, [.025, .975], method='linear').tolist()}
(OUT / 'reasoning_effort.json').write_text(json.dumps(output, indent=2) + '\n')
for row in rows:
    print(' | '.join(row))
print('High-minus-low Line F1 interval:', output['high_minus_low_ci'])
