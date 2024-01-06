fn main() {
    // I assumed passing a slice of chars would match if it started with any of
    // them, but there was the small chance they meant if the whole slice of
    // chars, interpreted as a string, was at the start of the target.
    assert!("abcd".starts_with(['z', 'a']));
}
