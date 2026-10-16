> If this PR is not adding a new vulnerability, you can ignore this template

---

### Checklist for adding a new vulnerability

- [ ] Created `samples/vuln-XXXX/` with the next free ID
- [ ] Added `sample-0XXXX-{crate,file,function}` (and `sample-1XXXX-…` if a fix exists), each with a matching package name in `Cargo.toml`
- [ ] Every crate compiles (`cargo build` at the workspace root)
- [ ] Shared dependency crates are in `samples/deps/` and listed in `deps`
- [ ] Wrote `samples/vuln-XXXX/README.md` from `samples/README_TEMPLATE.md`
- [ ] Added the entry to `mizan.json` (paths, `is_vulnerability`, CWE types, vulnerable functions and lines, `deps`)
- [ ] The JSON validation check passes

See [Add a vulnerability](https://sfu-rsl.github.io/rust-mizan/contributing/vulnerabilities.html).