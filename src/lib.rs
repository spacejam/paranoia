//! Paranoia is a simple hack to see if some code was optimized
//! away (by never being called) or not. This only serves as a
//! hint because sometimes the compiler isn't able to determine
//! whether code is able to be fully eliminated or not. But there
//! are no false negatives - if `marker_exists` returns false, you
//! can be certain that the call to marker was fully optimized away.
//!
//! # Examples
//!
//! `Cargo.toml`
//!
//! ```no_build
//! [dependencies]
//! paranoia-caller = "*"
//! paranoia = "*"
//! ```
//!
//! verify that it was optimized out:
//! ```
//! if false {
//!     paranoia_caller::mark();
//! }
//! assert!(!paranoia::marker_exists());
//! ```
//!
//! see if it was not able to be optimized out:
//! ```
//! if true {
//!     paranoia_caller::mark();
//! }
//! assert!(paranoia::marker_exists());
//! ```

fn exists() -> bool {
    use libc::{dlsym, RTLD_NEXT};
    use std::ffi::CString;

    let symbol = CString::new(b"___paranoia_present".to_vec()).unwrap();

    #[allow(unsafe_code)]
    let ptr = unsafe { dlsym(RTLD_NEXT, symbol.as_ptr()) };

    !ptr.is_null()
}

lazy_static::lazy_static! {
    static ref EXISTS: bool = exists();
}

/// Check to see if the marker has been optimized away or not.
pub fn marker_exists() -> bool {
    *EXISTS
}
