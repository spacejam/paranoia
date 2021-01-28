//! Paranoia is a simple hack to see if some code was optimized
//! away (by never being called) or not. This is not guaranteed
//! to work, and serves only as a hint. But there are no false
//! negatives - if `marker_exists` returns false, you can be
//! certain that the call to marker was fully optimized away.
//!
//! # Examples
//!
//! `Cargo.toml`
//!
//! ```no_build
//! [dependencies]
//! paranoia_caller = "*"
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
use libc::{dlsym, RTLD_NEXT};
use std::ffi::CString;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Once,
};

/// Check to see if the marker has been optimized away or not.
pub fn marker_exists() -> bool {
    static DLSYM: Once = Once::new();
    static EXISTS: AtomicBool = AtomicBool::new(true);

    DLSYM.call_once(|| {
        let symbol = CString::new(b"___paranoia_present".to_vec()).unwrap();

        #[allow(unsafe_code)]
        let ptr = unsafe { dlsym(RTLD_NEXT, symbol.as_ptr()) };

        EXISTS.store(!ptr.is_null(), Ordering::SeqCst)
    });

    EXISTS.load(Ordering::Relaxed)
}
