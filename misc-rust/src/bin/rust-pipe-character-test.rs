fn t(x: i32) -> bool {
    println!("true, {}", x);
    true
}

fn f(x: i32) -> bool {
    println!("false, {}", x);
    false
}

// I don't know why this was novel to me when I wrote this. I guess I was just
// surprised that booleans accept binary operators. In any case, this is only a
// recent commit because I want to delete the other top-level directory.
fn main() {
    println!("1 {}", t(1) | f(1));
    println!("2 {}", t(2) | t(2));
    println!("3 {}", f(3) | f(3));
    println!("4 {}", f(4) | t(4));
}
