use std::mem::MaybeUninit;

/// Guard that deallocates a `Vec` allocation whose ownership was taken via
/// `mem::forget`. It is used by `init_vec_with` to avoid leaking memory when
/// the initialization closure returns an error or panics.
struct DeallocGuard<T> {
    ptr: *mut T,
    cap: usize,
}

impl<T> DeallocGuard<T> {
    fn new(ptr: *mut T, cap: usize) -> Self {
        Self { ptr, cap }
    }

    /// Disarms the guard so it no longer deallocates the buffer.
    /// Call this once ownership of the allocation is transferred elsewhere.
    fn disable(mut self) {
        self.cap = 0;
    }
}

impl<T> Drop for DeallocGuard<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            unsafe {
                let _ = Vec::from_raw_parts(self.ptr, 0, self.cap);
            }
        }
    }
}

/// Allocates a `Vec<u8>` of `len` bytes without initializing it, calls `f` with
/// a mutable slice covering the whole buffer, and returns the vector after
/// checking that `f` wrote exactly `len` bytes.
///
/// # Safety
///
/// `f` must write a value to every byte of the provided slice. Returning fewer
/// bytes than `len` is a logic error and will panic in debug builds.
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn init_vec_with<E, F>(len: usize, f: F) -> Result<Vec<u8>, E>
where
    F: FnOnce(&mut [u8]) -> Result<usize, E>,
{
    let mut uninit: Vec<MaybeUninit<u8>> = Vec::with_capacity(len);
    let ptr = uninit.as_mut_ptr();
    let cap = uninit.capacity();
    std::mem::forget(uninit);

    let guard = DeallocGuard::new(ptr, cap);

    let written = f(std::slice::from_raw_parts_mut(ptr as *mut u8, len))?;

    debug_assert_eq!(
        written, len,
        "initialization function did not fill the whole buffer"
    );

    // Transfer ownership to the returned Vec; do not deallocate the buffer.
    guard.disable();
    Ok(Vec::from_raw_parts(ptr as *mut u8, len, cap))
}
