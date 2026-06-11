# Limitations

RustMizan makes some deliberate trade-offs. They are worth knowing before drawing conclusions from results.

- **Manually curated** The dataset is manually curated and verified to compile, which favors quality over quantity. It does not aim to cover every Rust vulnerability.
- **Labeling assumption.** Pre-patch code is treated as vulnerable and post-patch code as non-vulnerable. This follows standard practice in vulnerability research, but it assumes the patch resolves the intended issue and that no other vulnerability remains, which may not hold in every case.
- **Uneven mutation coverage.** Some mutations need specific constructs (loop rewrites need loops, conditional rewrites need branches), so a given variant is transformed only by the applicable operators. Contamination mitigation is therefore uneven across the dataset. The per-variant mutation log records which mutations were applied, so this is visible rather than hidden.
- **Published mutations and contamination.** Once mutated variants are released, they can be ingested into future training corpora and lose their contamination-testing value. The framework regenerates fresh variants on demand from the vanilla split to mitigate this, and contamination mitigation remains an active research area.
