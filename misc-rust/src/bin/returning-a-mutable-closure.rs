fn init_counter() -> impl FnMut() -> i32 {
    let mut a = 0;
    return move || {
        a += 1;
        return a;
    };
}

// This is a very basic example for how I learned about closures in other
// languages, and I realized two things this far along in my rust learning
// journey. 1) I've never felt compelled to write code in this sort of fashion.
// Then 2) that made me realize that I don't recall that last time I
// intentionally wrote code like that in other languages. Also, I just thought
// of 3) in reality a closure like this is simply an iterator in disguise. If
// it's not a externally "pure" iterator (returning a sequence of values without
// external side effects), then the closure constructor would have to take in
// the mutable object that it is going to apply side-effects to explicitly, e.g.
// adding a cache to a function would require holding a mutable reference to a
// HashMap object.
//
// I don't know if this offers much insight, but I guess the main takeaway is
// that closures are just one way to implement various patterns.
fn main() {
    let mut counter = init_counter();
    println!("{}", counter());
    println!("{}", counter());
    println!("{}", counter());
}
