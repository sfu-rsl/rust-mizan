"""Dataset counts, code sizes, years, CWEs, and reduction coverage."""
import json
from collections import Counter
import pandas as pd
from common import CODE, OUT, ROOT, write_csv

data = pd.read_parquet(ROOT / 'evals/mizan-vanilla.parquet')
registry = json.loads((CODE / 'mizan.json').read_text())['vulnerabilities']
data['rust_loc'] = [sum(f['content'].count('\n') + 1 for f in files
                        if f['path'].endswith('.rs')) for files in data['files']]
assert set(data.vuln_id) == {v['id'] for v in registry}
counts = Counter()
for vuln in registry:
    counts.update({cwe for sample in vuln['code_samples'] for cwe in sample['cwe_type']})
stats = {
    'vulnerabilities': len(registry), 'crates': int(data.crate_name.nunique()),
    'variants': len(data), 'vulnerable': int(data.is_vulnerable.sum()),
    'patched': int((~data.is_vulnerable).sum()),
    'vulnerabilities_with_patched_variants': int(data[~data.is_vulnerable].vuln_id.nunique()),
    'variants_by_level': data.granularity.value_counts().to_dict(),
    'median_rust_loc_by_level': data.groupby('granularity').rust_loc.median().to_dict(),
    'vulnerabilities_by_crate': data.groupby('crate_name').vuln_id.nunique().to_dict(),
    'vulnerabilities_by_year': dict(Counter(v['year'] for v in registry)),
    'vulnerabilities_by_cwe': dict(counts),
}
(OUT / 'dataset_statistics.json').write_text(json.dumps(stats, indent=2) + '\n')
write_csv('dataset_samples.csv', ['sample_id', 'crate', 'level', 'vulnerable', 'rust_loc'],
          data[['sample_id', 'crate_name', 'granularity', 'is_vulnerable', 'rust_loc']].values)
print(json.dumps(stats, indent=2))
