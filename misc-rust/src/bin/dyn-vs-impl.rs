use std::fmt::Debug;

fn expect_dyn(obj: Box<dyn Debug>) {
    println!("dyn obj = {:?}", obj)
}

fn expect_impl(obj: impl Debug) {
    println!("impl obj = {:?}", obj)
}

fn return_dyn() -> Box<dyn Debug> {
    Box::new("asdf_dyn")
}

fn return_impl() -> impl Debug {
    "asdf_impl"
}

fn main() {
    expect_dyn(return_dyn());
    expect_dyn(Box::new(return_impl())); // This is not surprising to me
    expect_impl(return_dyn()); // I'm sorta surprised this works though. In hindsight, I guess it makes sense. The automatic smart-pointer dereferencing is amazing!
    expect_impl(return_impl());
}
