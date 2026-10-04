"""Project sizes from vanilla inputs and the paper's dated download snapshot."""
import json
import pandas as pd
from common import ROOT, write_csv, write_table

data = pd.read_parquet(ROOT / 'evals/mizan-vanilla.parquet')
snapshot = json.loads((ROOT / 'project_downloads.json').read_text())
data['rust_loc'] = [sum(f['content'].count('\n') + 1 for f in files
                        if f['path'].endswith('.rs')) for files in data['files']]
sizes = data[data.granularity == 'crate'].groupby('crate_name').rust_loc.max().to_dict()
assert set(sizes) == set(snapshot['downloads'])
rows = [[name, sizes[name], snapshot['downloads'][name]] for name in sorted(sizes)]
write_csv('project_statistics.csv', ['crate', 'rust_loc', 'downloads'], rows)
write_table('project_statistics.tex', ['Crate', 'Rust LOC', 'Downloads'],
            [[r'\texttt{' + name.replace('_', r'\_') + '}', f'{loc:,}', f'{dl:,}']
             for name, loc, dl in rows])
print(f'{len(rows)} crates; downloads retrieved {snapshot["retrieved_at_utc"]}')
