// TODO: Potentially unclear example. I am just committing this, because I'm tired of so many leftover git stashes

macro_rules! repeat_two {
    (($($i:ident,)*) $($i2:ident)*) => {
        $( let $i: (); let $i2: (); )*
    }
}

fn main() {
    repeat_two!( (a, b, c, d, e, f) u v w x y z );
}
