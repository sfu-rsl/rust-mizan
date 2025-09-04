## Kani

[CVE-2020-25791](https://rustsec.org/advisories/RUSTSEC-2020-0041.html) is an out-of-bounds write vulnerability in the `sized-chunks` crate. The vulnerable `Chunk::unit()` function lacks proper capacity validation which allows unsafe pointer arithmetic to write beyond allocated memory bounds when using zero-capacity chunks (`UTerm`).

### Test Vulnerable Code Samples

> Make sure you have Kani installed. Follow the [the docs](https://model-checking.github.io/kani/install-guide.html)

All vulnerable samples should detect the same out-of-bounds write vulnerability:

```bash
# Test vulnerable crate sample
cargo kani --harness test_vulnerable_crate_zero_capacity_chunk

# Test vulnerable file sample
cargo kani --harness test_vulnerable_file_zero_capacity_chunk

# Test vulnerable function sample
cargo kani --harness test_vulnerable_function_zero_capacity_chunk
```

Expected output for vulnerable samples (truncated):

```sh
Check X: std::ptr::write::<i32>.pointer_dereference.5
	 - Status: FAILURE
	 - Description: "dereference failure: pointer outside object bounds"
	 - Location: ../../../../../../../../../runner/.rustup/toolchains/nightly-2024-09-03-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1646:9 in function std::ptr::write::<i32>

# ... additional output truncated ...

SUMMARY:
 ** 1 of 36 failed (2 unreachable)
Failed Checks: dereference failure: pointer outside object bounds
 File: "/Users/runner/.rustup/toolchains/nightly-2024-09-03-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs", line 1646, in std::ptr::write::<i32>

VERIFICATION:- FAILED
Verification Time: 0.042307418s

Summary:
Verification failed for - tests::test_vulnerable_crate_zero_capacity_chunk
Complete - 0 successfully verified harnesses, 1 failures, 1 total.

```

### Test Fixed Code Samples

All fixed samples should fail at the capacity assertion (which is correct behavior):

```bash
# Test fixed crate sample
cargo kani --harness test_fixed_crate_zero_capacity_chunk

# Test fixed file sample
cargo kani --harness test_fixed_file_zero_capacity_chunk

# Test fixed function sample
cargo kani --harness test_fixed_function_zero_capacity_chunk
```

Expected output for fixed samples (truncated):

```sh
Check X: std::ptr::write::<i32>.pointer_dereference.5
	 - Status: SUCCESS
	 - Description: "dereference failure: pointer outside object bounds"
	 - Location: ../../../../../../../../../runner/.rustup/toolchains/nightly-2024-09-03-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:1646:9 in function std::ptr::write::<i32>

# ... additional output truncated ...

SUMMARY:
 ** 1 of 37 failed (3 unreachable)
Failed Checks: assertion failed: Self::CAPACITY >= 1
 File: "/Users/tareknasser/Documents/workspace/sfu/mizan/rust-mizan/samples/vuln-0018/sample-10018-crate/src/sized_chunk/mod.rs", line 158, in sample_10018_crate::Chunk::<i32, typenum::UTerm>::unit

VERIFICATION:- FAILED
Verification Time: 0.012982s

Summary:
Verification failed for - tests::test_fixed_crate_zero_capacity_chunk
Complete - 0 successfully verified harnesses, 1 failures, 1 total.
```
