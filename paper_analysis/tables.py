"""Main scores and mutation comparison from saved scorer counters."""
from common import MODEL_ORDER, NAMES, SPLIT_ORDER, load_logs, micro, write_csv, write_table

logs = load_logs()
scores = {}
csv_rows = []
for split in SPLIT_ORDER:
    for model in MODEL_ORDER:
        rows = list(logs[(split, model)].values())
        n_vulnerable = sum(r['is_vulnerable_gt'] for r in rows)
        counts = [sum(r[k] for r in rows) for k in
                  ('binary_accuracy', 'success_at_1_function', 'success_at_1_line')]
        values = [100 * counts[0] / len(rows)]
        values += [micro(rows, task)[2] for task in ('cwe', 'function', 'line')]
        values += [100 * n / n_vulnerable for n in counts[1:]]
        scores[split, model] = values
        csv_rows.append([split, NAMES[model], len(rows), n_vulnerable, *values])

write_csv('metrics.csv', ['split', 'model', 'samples', 'vulnerable_samples',
                         'detection_accuracy', 'cwe_f1', 'function_f1', 'line_f1',
                         'success_at_1_function', 'success_at_1_line'], csv_rows)
rows = []
for model in MODEL_ORDER:
    records = list(logs['vanilla', model].values())
    values = [f'{v:.1f}' for v in scores['vanilla', model]]
    for column, field, denominator in ((0, 'binary_accuracy', len(records)),
                                        (4, 'success_at_1_function', 95),
                                        (5, 'success_at_1_line', 95)):
        values[column] += f' ({sum(r[field] for r in records)}/{denominator})'
    rows.append([NAMES[model], *values])
write_table('main_results.tex', ['Model', 'CVC Acc.', 'CWE F1', 'Function F1',
                                 'Line F1', 'Function S@1', 'Line S@1'], rows)

rows = [[NAMES[m], *[f'{scores[s,m][3]:.1f}' for s in SPLIT_ORDER]] for m in MODEL_ORDER]
changes = [sum(scores[s,m][3] - scores['vanilla',m][3] for m in MODEL_ORDER) / 4
           for s in SPLIT_ORDER[1:]]
rows.append(['Mean change', '--', *[f'{v:+.2f}' for v in changes]])
write_table('mutation_results.tex', ['Model', 'Vanilla', 'Benign', 'Rust-Specific', 'Malignant'], rows)
for row in rows:
    print(' | '.join(row))
