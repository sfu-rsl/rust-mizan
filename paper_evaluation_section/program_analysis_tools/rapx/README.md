## RAPx

> Make sure you have RAPx installed and configured. See the [guide](https://github.com/Artisan-Lab/RAPx?tab=readme-ov-file#quick-start)

> All the commands below should be run from the root of the `rust-mizan` repository.

### Test Vulnerable Code Samples

All vulnerable samples should detect the same dangling pointer vulnerability in the `from` function:

```bash
# Test vulnerable crate sample
cargo +nightly-2025-08-17 rapx -F -M -- -p sample-00001-crate

# Test vulnerable file sample
cargo +nightly-2025-08-17 rapx -F -M -- -p sample-00001-file

# Test vulnerable function sample
cargo +nightly-2025-08-17 rapx -F -M -- -p sample-00001-function
```

Expected output for vulnerable samples (truncated):

```sh
13:52:01|RAP|INFO|: Intrinsic = {
    "std::alloc::dealloc": DefId(3:134 ~ alloc[b3ac]::alloc::dealloc),
    "std::clone::Clone::clone": DefId(2:3154 ~ core[800a]::clone::Clone::clone),
    "std::mem::ManuallyDrop::<T>::drop": DefId(2:2134 ~ core[800a]::mem::manually_drop::{impl#1}::drop),
    "std::mem::MaybeUninit::<T>::assume_init_drop": DefId(2:2169 ~ core[800a]::mem::maybe_uninit::{impl#2}::assume_init_drop),
    "std::mem::drop": DefId(2:2330 ~ core[800a]::mem::drop),
    "std::ops::FnMut::call_mut": DefId(2:3983 ~ core[800a]::ops::function::FnMut::call_mut),
    "std::ptr::const_ptr::<impl *const T>::copy_to": DefId(2:2781 ~ core[800a]::ptr::const_ptr::{impl#0}::copy_to),
    "std::ptr::const_ptr::<impl *const T>::copy_to_nonoverlapping": DefId(2:2782 ~ core[800a]::ptr::const_ptr::{impl#0}::copy_to_nonoverlapping),
    "std::ptr::drop_in_place": DefId(2:2973 ~ core[800a]::ptr::drop_in_place),
    "std::ptr::mut_ptr::<impl *mut T>::copy_from": DefId(2:2903 ~ core[800a]::ptr::mut_ptr::{impl#0}::copy_from),
    "std::ptr::mut_ptr::<impl *mut T>::copy_from_nonoverlapping": DefId(2:2904 ~ core[800a]::ptr::mut_ptr::{impl#0}::copy_from_nonoverlapping),
}
13:53:46|RAP|WARN|: Dangling pointer detected in function "from"
warning: Dangling pointer detected.
   --> samples/vuln-0001/sample-00001-crate/src/buffer.rs:191:1
    |
191 | / fn from(buffer: Buffer) -> Vec<u8> {
192 | |         let mut slice = Buffer::allocate(buffer.len);
193 | |         let len = buffer.copy_to(&mut slice);
194 | |
195 | |         unsafe { Vec::from_raw_parts(slice.as_mut_ptr(), len, slice.len()) }
196 | |     }
    | |_____- Dangling pointer detected.
    |
Over visited: DefId(0:194 ~ sample_00001_crate[5633]::transport::{impl#0}::begin_request)
```

### Test Fixed Code Samples

Fixed samples should complete analysis without detecting dangling pointer vulnerabilities:

```bash
# Test fixed crate sample
cargo +nightly-2025-08-17 rapx -F -M -- -p sample-10001-crate

# Test fixed file sample
cargo +nightly-2025-08-17 rapx -F -M -- -p sample-10001-file

# Test fixed function sample
cargo +nightly-2025-08-17 rapx -F -M -- -p sample-10001-function
```

Expected output for fixed samples (truncated):

```sh
13:54:33|RAP|INFO|: Intrinsic = {
    "std::alloc::dealloc": DefId(3:134 ~ alloc[b3ac]::alloc::dealloc),
    "std::clone::Clone::clone": DefId(2:3154 ~ core[800a]::clone::Clone::clone),
    "std::mem::ManuallyDrop::<T>::drop": DefId(2:2134 ~ core[800a]::mem::manually_drop::{impl#1}::drop),
    "std::mem::MaybeUninit::<T>::assume_init_drop": DefId(2:2169 ~ core[800a]::mem::maybe_uninit::{impl#2}::assume_init_drop),
    "std::mem::drop": DefId(2:2330 ~ core[800a]::mem::drop),
    "std::ops::FnMut::call_mut": DefId(2:3983 ~ core[800a]::ops::function::FnMut::call_mut),
    "std::ptr::const_ptr::<impl *const T>::copy_to": DefId(2:2781 ~ core[800a]::ptr::const_ptr::{impl#0}::copy_to),
    "std::ptr::const_ptr::<impl *const T>::copy_to_nonoverlapping": DefId(2:2782 ~ core[800a]::ptr::const_ptr::{impl#0}::copy_to_nonoverlapping),
    "std::ptr::drop_in_place": DefId(2:2973 ~ core[800a]::ptr::drop_in_place),
    "std::ptr::mut_ptr::<impl *mut T>::copy_from": DefId(2:2903 ~ core[800a]::ptr::mut_ptr::{impl#0}::copy_from),
    "std::ptr::mut_ptr::<impl *mut T>::copy_from_nonoverlapping": DefId(2:2904 ~ core[800a]::ptr::mut_ptr::{impl#0}::copy_from_nonoverlapping),
}
Over visited: DefId(0:194 ~ sample_10001_crate[3cc1]::transport::{impl#0}::begin_request)
```
