#[test]
fn not_present() {
    if false {
        paranoia_caller::mark();
    }
    assert!(!paranoia::marker_exists());
}
