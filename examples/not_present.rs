fn main() {
    if false {
        paranoia_caller::mark();
    }
    assert!(!paranoia::marker_exists());
}
