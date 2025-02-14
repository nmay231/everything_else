// TODO: Potentially unclear example. I am just committing this, because I'm tired of so many leftover git stashes

// TODO: This makes sense now because what's really happening, is sometimes you
// have variables that become `&'static Type` which is always safe to return
// since it's an immutable static variable. Returning a reference to an owned
// value fails because it's dropped

// I need to commit this, but I want to clean up the examples a bit.
// JK, I'm gonna commit this now that it's sat in my git stashes for way too long.

struct Testing;

// When learning more about life-times
fn testing1<'a>() -> &'a Testing {
    &Testing
}

#[inline(never)]
fn testing2<'a>() -> &'a Testing {
    &Testing
}

fn testing2b<'a>() -> &'a Testing {
    if true {
        &Testing
    } else {
        &Testing
    }
}

fn testing1a<'a>() -> &'a Testing {
    let var = &Testing;
    var
}

fn testing1b<'a>() -> &'a Testing {
    let var = Testing;
    &var
}

fn testing1c<'a>() -> &'a mut Testing {
    &mut Testing
}

struct Testing2(i32);

// fn testing2<'a>() -> &'a Testing2 {
//     let x = Testing2(0);
//     &x
// }

#[derive(Clone)]
struct Uncopyable;
struct Testing3(Uncopyable);

fn testing3a<'a>() -> &'a Testing3 {
    &Testing3(Uncopyable)
}

fn testing3b<'a>() -> &'a Uncopyable {
    &Testing3(Uncopyable).0
}

fn main() {}
