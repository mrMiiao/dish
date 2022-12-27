pub use core::mem::*;

/// Raw value comparison.
pub fn eq<T>(a: &T, b: &T) -> bool {
    unsafe {
        core::intrinsics::raw_eq(a, b)
    }
}