"""Summarize archived regression counts for the operators used in the paper."""
import re
from common import ROOT, write_csv, write_table

report = (ROOT / 'preservation_counts.txt').read_text()
crates = ['itertools', 'num-traits', 'num-bigint', 'byteorder']
baseline = dict((crate, int(count)) for crate, count in
                re.findall(r'^BASELINE ([\w-]+) (\d+)$', report, re.M))
mutations = ['for-to-while', 'while-to-loop', 'if-else-reorder', 'derive-reorder',
             'trait-bound-reorder', 'use-reorder', 'arithmetic-identity', 'explicit-where',
             'rename-lifetime', 'extraneous-unsafe', 'impl-trait-to-generic', 'option-wrap',
             'maybeuninit-wrap', 'manuallydrop-wrap', 'explicit-return',
             'unreachable-panic', 'repeated-shadowing']
results = {}
for crate, mutation, status, count in re.findall(
        r'^\| ([\w-]+) \| `([\w-]+)` \| (PASS|FAIL)(?: \((\d+)\))? \|$', report, re.M):
    if mutation in mutations:
        if status == 'PASS':
            assert int(count) == baseline[crate]
        results[crate, mutation] = status
assert len(results) == 68
rows = [[mutation, *[results[crate, mutation] for crate in crates]] for mutation in mutations]
write_csv('preservation.csv', ['Mutation', *crates], rows)
# Archived FAIL cells were compilation failures, as checked for the paper.
table = [[r'\texttt{' + row[0] + '}', *['CF' if x == 'FAIL' else x for x in row[1:]]]
         for row in rows]
passing = [sum(results[crate, m] == 'PASS' for m in mutations) for crate in crates]
tests = [passing[i] * baseline[crate] for i, crate in enumerate(crates)]
table += [['Passing combinations', *passing], ['Tests after mutation', *tests]]
write_table('preservation.tex', ['Mutation', *crates], table)
print(f'{sum(passing)} of 68 combinations pass; {sum(tests):,} post-mutation test executions')
