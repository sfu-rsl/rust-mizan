"""Per-vulnerability line localization at each code level"""
import json
import re
from common import CODE, load_logs, write_csv, write_table

logs = load_logs()
registry = json.loads((CODE / 'mizan.json').read_text())['vulnerabilities']
models = ['GPT', 'Gemini', 'Claude', 'Qwen']
levels = ['crate', 'file', 'function']
rows = []
for vuln in registry:
    identifier = re.search(r'(?:CVE|RUSTSEC)-\d{4}-\d+', vuln['source_link']).group()
    vulnerable = [s for s in vuln['code_samples'] if s['is_vulnerability']]
    cwe = vulnerable[0]['cwe_type'][0]
    cells = []
    for model in models:
        for level in levels:
            records = [r for r in logs['vanilla', model].values()
                       if r['vuln_id'] == vuln['id'] and r['granularity'] == level
                       and r['is_vulnerable_gt']]
            assert len(records) <= 1
            status = 'absent' if not records else (
                'invalid' if not records[0]['valid'] else (
                    'yes' if records[0]['line_tp'] > 0 else 'no'))
            cells.append(status)
    rows.append([cwe, identifier, *cells])
rows.sort(key=lambda row: (row[0], row[1]))
header = ['CWE', 'Vulnerability'] + [f'{m} {level}' for m in models for level in levels]
write_csv('per_sample_results.csv', header, rows)
symbols = {'yes': r'\checkmark', 'no': r'$\times$', 'absent': '--', 'invalid': 'invalid'}
write_table('per_sample_results.tex', header,
            [[*row[:2], *[symbols[v] for v in row[2:]]] for row in rows])
print(f'{len(rows)} vulnerabilities, {len(rows) * 12} model/level cells')
