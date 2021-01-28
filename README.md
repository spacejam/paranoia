# paranoia

How it works:

1. The `paranoia-sys` crate is a cdylib that exports this function using dynamic linking:
    ```rust
    #[no_mangle]
    #[inline(never)]
    pub extern "C" fn ___paranoia_present() {
        #[allow(unsafe_code)]
        unsafe {
            std::ptr::read_volatile(&());
        }
    }
    ```
2. The `paranoia-caller` crate dynamically links against that code:
    ```rust
    #[link(name = "paranoia_sys")]
    extern "C" {
        fn ___paranoia_present();
    }

    /// Mark a branch that can be later tested to see if it
    /// was compiled away or not.
    pub fn mark() {
        unsafe {
            ___paranoia_present();
        }
    }
    ```
3. The `paranoia` crate uses dlsym to check at runtime if the symbol is present:
    ```rust
    fn exists() -> bool {
        let symbol = CString::new(b"___paranoia_present".to_vec()).unwrap();

        let ptr = unsafe { dlsym(RTLD_NEXT, symbol.as_ptr()) };

        !ptr.is_null()
    }
    ```

So, we can use this to essentially access information about what compilation did or did not optimize away at runtime. This is handy for avoiding compensatory effort that only needs to be spent when certain functionality is invoked, but you don't want to use cargo features for whatever reason.

Note that this doesn't work quite as intended for the time being - we need to use a build.rs similar to other `*-sys` crates that build shared objects and uses them to link against, and right now this just serves as a simple proof of concept that you can do this kind of thing at all.
