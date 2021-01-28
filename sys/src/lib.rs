/// This function exists as a marker that will be compiled away
/// if it is never called from a final compiled binary. Its
/// existence is tested using dlsym when running the final
/// binary.
#[no_mangle]
#[inline(never)]
pub extern "C" fn ___paranoia_present() {
    #[allow(unsafe_code)]
    unsafe {
        std::ptr::read_volatile(&());
    }
}
