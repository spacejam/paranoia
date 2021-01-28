#[test]
fn present() {
    if true {
        paranoia_caller::mark();
    }
    assert!(paranoia::marker_exists());
}
