/// https://docs.rs/typenum/latest/typenum/uint/struct.UTerm.html
/// The terminating type for UInt; it always comes after the most significant bit. UTerm by itself represents zero, which is aliased to U0.
use typenum::UTerm;

// Vulnerable samples
use sample_00018_crate::Chunk as VulnCrateChunk;
use sample_00018_file::Chunk as VulnFileChunk;
use sample_00018_function::Chunk as VulnFunctionChunk;

// Fixed samples  
use sample_10018_crate::Chunk as FixedCrateChunk;
use sample_10018_file::Chunk as FixedFileChunk;
use sample_10018_function::Chunk as FixedFunctionChunk;

// Test vulnerable samples - should all detect the out-of-bounds write vulnerability
#[kani::proof]
fn test_vulnerable_crate_zero_capacity_chunk() {
    let symbolic_value: i32 = kani::any();
    let _chunk = VulnCrateChunk::<i32, UTerm>::unit(symbolic_value);
}

#[kani::proof]
fn test_vulnerable_file_zero_capacity_chunk() {
    let symbolic_value: i32 = kani::any();
    let _chunk = VulnFileChunk::<i32, UTerm>::unit(symbolic_value);
}

#[kani::proof]
fn test_vulnerable_function_zero_capacity_chunk() {
    let symbolic_value: i32 = kani::any();
    let _chunk = VulnFunctionChunk::<i32, UTerm>::unit(symbolic_value);
}

// Test fixed samples - should all fail at the capacity assertion (correct behavior)
#[kani::proof]
fn test_fixed_crate_zero_capacity_chunk() {
    let symbolic_value: i32 = kani::any();
    let _chunk = FixedCrateChunk::<i32, UTerm>::unit(symbolic_value);
}

#[kani::proof]
fn test_fixed_file_zero_capacity_chunk() {
    let symbolic_value: i32 = kani::any();
    let _chunk = FixedFileChunk::<i32, UTerm>::unit(symbolic_value);
}

#[kani::proof]
fn test_fixed_function_zero_capacity_chunk() {
    let symbolic_value: i32 = kani::any();
    let _chunk = FixedFunctionChunk::<i32, UTerm>::unit(symbolic_value);
}
