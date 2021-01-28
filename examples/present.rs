fn main() {
    paranoia_caller::mark();
    assert!(paranoia::marker_exists());
}
