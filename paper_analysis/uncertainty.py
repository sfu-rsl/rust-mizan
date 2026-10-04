"""Camera-ready bootstrap estimates from the original evaluation logs."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import zipfile

import numpy as np

ROOT = Path(__file__).resolve().parent
OUT = ROOT / 'output'
OUT.mkdir(exist_ok=True)
MODELS = ('anthropic/claude-sonnet-4-6', 'openai/gpt-5.4',
          'google/gemini-3.1-pro-preview', 'openrouter/qwen/qwen3.6-plus')
NAMES = ('Claude Sonnet 4.6', 'GPT 5.4', 'Gemini 3.1 Pro', 'Qwen 3.6 Plus')
SPLITS = ('vanilla', 'benign', 'rust-specific', 'malignant')
METRICS = ('detection_accuracy', 'cwe_f1', 'function_f1', 'line_f1',
           'success_at_1_function', 'success_at_1_line')
FIELDS = ('cwe_tp', 'cwe_fp', 'cwe_fn', 'function_tp', 'function_fp',
          'function_fn', 'line_tp', 'line_fp', 'line_fn')
RUNS, FILES = {}, []


def elements(mapping):
    return {(file, item) for file, items in mapping.items() for item in items}


for path in sorted((ROOT / 'evals').glob('*.eval')):
    with zipfile.ZipFile(path) as archive:
        h = json.loads(archive.read('header.json'))
        model = h['eval']['model']
        split = h['eval']['task'].removeprefix('mizan-')
        assert model in MODELS and split in SPLITS
        assert h['status'] == 'success' and h['eval']['config']['epochs'] == 1
        key = model, split
        assert key not in RUNS, ('Duplicate run', key)
        rows = {}
        for member in archive.namelist():
            if not (member.startswith('samples/') and member.endswith('.json')):
                continue
            s = json.loads(archive.read(member))
            sid, metadata = s['id'], s['metadata']
            assert sid not in rows and s['epoch'] == 1
            assert metadata['sample_id'] == sid
            assert metadata['vuln_id'] == sid.split('/')[0]
            score = s['scores']['rustmizan_scorer']
            c = score['metadata']
            assert c['is_vulnerable_gt'] == metadata['is_vulnerable']
            gt = bool(metadata['is_vulnerable'])
            if score['answer'] == 'valid':
                p = c['parsed_response']
                assert c['binary_accuracy'] == int(p['is_vulnerable'] == gt)
                for prefix, predicted, actual in (
                    ('cwe', set(p['cwe_type']), set(metadata['cwe_type'])),
                    ('function', elements(p['vulnerable_functions']), elements(metadata['vulnerable_functions'])),
                    ('line', elements(p['vulnerable_lines']), elements(metadata['vulnerable_lines'])),
                ):
                    expected = (len(predicted & actual), len(predicted - actual), len(actual - predicted))
                    assert expected == tuple(c[prefix + '_' + k] for k in ('tp', 'fp', 'fn')), (path.name, sid, prefix)
                assert c['success_at_1_function'] == int(gt and c['function_tp'] > 0)
                assert c['success_at_1_line'] == int(gt and c['line_tp'] > 0)
            else:
                assert all(c[k] == 0 for k in ('binary_accuracy', *FIELDS,
                                              'success_at_1_function', 'success_at_1_line'))
            vector = (1, int(gt), c['binary_accuracy'], *(c[k] for k in FIELDS),
                      c['success_at_1_function'], c['success_at_1_line'])
            assert all(x >= 0 and int(x) == x for x in vector)
            rows[sid] = {'cluster': metadata['vuln_id'],
                         'identity': [metadata[k] for k in ('vuln_id', 'crate_name', 'granularity', 'year', 'is_vulnerable', 'cwe_type')],
                         'counters': list(map(int, vector)), 'answer': score['answer']}
        assert len(rows) == h['results']['completed_samples'] == 173
        assert sum(r['counters'][1] for r in rows.values()) == 95
        RUNS[key] = rows
        FILES.append({'filename': path.name, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                      'model': model, 'split': split, 'invalid': sum(r['answer'] != 'valid' for r in rows.values())})

assert set(RUNS) == {(m, s) for m in MODELS for s in SPLITS}
BASE = RUNS[(MODELS[0], SPLITS[0])]
IDS = sorted(BASE)
CVES = sorted({r['cluster'] for r in BASE.values()})
assert len(CVES) == 42
for key, rows in RUNS.items():
    assert set(rows) == set(IDS), key
    assert all(rows[sid]['identity'] == BASE[sid]['identity'] for sid in IDS), key
registry = json.loads((ROOT.parent / 'mizan.json').read_text())['vulnerabilities']
sources = {v['id']: v['source_link'] for v in registry if v['id'] in CVES}
assert set(sources) == set(CVES) and len(set(sources.values())) == len(CVES)
assert all(re.search(r'CVE-\d{4}-\d+|RUSTSEC-\d{4}-\d+', src) for src in sources.values()), sources

TENSOR = np.zeros((4, 4, len(CVES), 14), dtype=np.int64)
for mi, model in enumerate(MODELS):
    for si, split in enumerate(SPLITS):
        for row in RUNS[(model, split)].values():
            TENSOR[mi, si, CVES.index(row['cluster'])] += row['counters']


def metrics(sums):
    """Recompute ratios of total counts, never average per-CVE F1."""
    result = [100 * sums[..., 2] / sums[..., 0]]
    for i in (3, 6, 9):
        denominator = 2 * sums[..., i] + sums[..., i + 1] + sums[..., i + 2]
        result.append(np.divide(200 * sums[..., i], denominator,
                               out=np.full(denominator.shape, 100.0), where=denominator != 0))
    assert np.all(sums[..., 1] > 0)
    result += [100 * sums[..., i] / sums[..., 1] for i in (12, 13)]
    return np.stack(result, axis=-1)


OBSERVED = metrics(TENSOR.sum(axis=2))
assert np.array_equal(np.round(OBSERVED[:, 0], 1), np.array([
    [64.7, 17.0, 21.3, 23.2, 46.3, 47.4],
    [64.2, 16.0, 15.7, 17.5, 46.3, 46.3],
    [57.2, 22.1, 22.4, 23.1, 52.6, 36.8],
    [56.1, 13.7, 24.6, 20.2, 44.2, 43.2],
]))
assert np.array_equal(np.round(OBSERVED[..., 3], 1), np.array([
    [23.2, 19.7, 24.0, 12.8], [17.5, 16.4, 15.5, 14.3],
    [23.1, 20.7, 22.2, 19.7], [20.2, 25.2, 19.6, 14.7],
]))


def compute(n, seed):
    rng = np.random.default_rng(seed)
    draws = np.empty((n, 4, 4, 6))
    for start in range(0, n, 5000):
        count = min(5000, n - start)
        # Whole CVEs sampled uniformly with replacement, same draw for every condition/model.
        weights = rng.multinomial(len(CVES), np.full(len(CVES), 1 / len(CVES)), size=count)
        summed = np.einsum('bg,msgk->bmsk', weights, TENSOR, optimize=True)
        if start == 0:
            # Independent sample-expansion implementation checks multiplicity and micro aggregation.
            for i in range(25):
                selected = [sid for gi, cve in enumerate(CVES) for _ in range(weights[i, gi])
                            for sid in IDS if BASE[sid]['cluster'] == cve]
                for mi, model in enumerate(MODELS):
                    for si, split in enumerate(SPLITS):
                        brute = np.array([RUNS[(model, split)][sid]['counters'] for sid in selected]).sum(axis=0)
                        assert np.array_equal(brute, summed[i, mi, si])
        draws[start:start + count] = metrics(summed)
    assert np.isfinite(draws).all() and (draws >= 0).all() and (draws <= 100).all()
    delta = draws[:, :, 1:, 3] - draws[:, :, :1, 3]
    mean_delta = delta.mean(axis=1)
    mean_scores = draws[:, :, :, 3].mean(axis=1)
    relative_drop = 100 * (mean_scores[:, 0] - mean_scores[:, 3]) / mean_scores[:, 0]
    gap = draws[:, :, 0, 0] - draws[:, :, 0, 3]
    ci = lambda a: np.quantile(a, (0.025, 0.975), axis=0, method='linear').tolist()
    return {'resamples': n, 'seed': seed, 'score_ci': ci(draws),
            'delta_ci': ci(delta), 'mean_delta_ci': ci(mean_delta),
            'relative_drop_ci': ci(relative_drop), 'gap_ci': ci(gap)}


primary = compute(200000, 20261004)
delta = OBSERVED[:, 1:, 3] - OBSERVED[:, :1, 3]
mean = OBSERVED[..., 3].mean(axis=0)
result = {'models': NAMES, 'model_ids': MODELS, 'splits': SPLITS, 'metrics': METRICS,
          'clusters': CVES, 'cve_sources': sources, 'cluster_size_histogram': dict(Counter(sum(BASE[sid]['cluster'] == cve for sid in IDS) for cve in CVES)),
          'files': FILES, 'observed': OBSERVED.tolist(), 'delta': delta.tolist(),
          'mean_delta': delta.mean(axis=0).tolist(), 'mean_line_f1': mean.tolist(),
          'relative_drop': float(100 * (mean[0] - mean[3]) / mean[0]),
          'primary': primary}
(OUT / 'uncertainty.json').write_text(json.dumps(result, indent=2) + '\n')
for mi, name in enumerate(NAMES):
    print(name)
    for ki, metric in enumerate(METRICS):
        lo, hi = (primary['score_ci'][b][mi][0][ki] for b in (0, 1))
        print(' ', metric, f'{OBSERVED[mi,0,ki]:.4f} [{lo:.4f}, {hi:.4f}]')
    for si, split in enumerate(SPLITS[1:]):
        lo, hi = (primary['delta_ci'][b][mi][si] for b in (0, 1))
        print(' ', split, f'{delta[mi,si]:+.4f} [{lo:+.4f}, {hi:+.4f}]')
for si, split in enumerate(SPLITS[1:]):
    print('MEAN', split, delta.mean(axis=0)[si], [primary['mean_delta_ci'][b][si] for b in (0, 1)])
print('RELATIVE DROP', result['relative_drop'], primary['relative_drop_ci'])
print('PASS: all 2,768 scorer records checked; matching CVEs/variants; reported points reproduced; expanded-sample cross-check passed')

from common import write_table
interval = lambda lo, hi: f"$[{lo:.1f}, {hi:.1f}]$"
rows = [[name] + [interval(primary['score_ci'][0][mi][0][ki],
                          primary['score_ci'][1][mi][0][ki]) for ki in range(6)]
        for mi, name in enumerate(NAMES)]
write_table('vanilla_intervals.tex', ['Model', 'CVC Acc.', 'CWE F1', 'Function F1',
                                     'Line F1', 'Function S@1', 'Line S@1'], rows)
rows = [[name] + [interval(primary['score_ci'][0][mi][si][3],
                          primary['score_ci'][1][mi][si][3]) for si in range(4)]
        for mi, name in enumerate(NAMES)]
write_table('mutation_intervals.tex', ['Model', 'Vanilla', 'Benign', 'Rust-Specific', 'Malignant'], rows)
rows = []
for si, split in enumerate(SPLITS[1:]):
    for mi, name in enumerate(NAMES):
        lo, hi = (primary['delta_ci'][b][mi][si] for b in (0, 1))
        rows.append([split, name, f"{delta[mi,si]:+.2f}", f"$[{lo:.2f}, {hi:.2f}]$"])
    lo, hi = (primary['mean_delta_ci'][b][si] for b in (0, 1))
    rows.append([split, 'Four-model mean', f"{delta.mean(axis=0)[si]:+.2f}", f"$[{lo:.2f}, {hi:.2f}]$"])
write_table('mutation_changes.tex', ['Condition', 'Model', 'Change (points)', r'95\% interval'], rows)
