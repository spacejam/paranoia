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
