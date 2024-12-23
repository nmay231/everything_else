use std::collections::HashMap;

struct Test;
impl Test {
    pub fn return_map_of_t<T>() -> HashMap<T, T>
    where
        for<'a> T: 'a,
    {
        HashMap::new()
    }
}

#[derive(PartialEq, Eq, Hash)]
struct DummyStruct(i32);

// TODO: Write down the youtube video, spend no more than 15 minutes trying to
// get a working example on my own, then copy his example if need be
fn main() {
    let mut x = Test::return_map_of_t();
    let key = DummyStruct(1);
    {
        let value = DummyStruct(2);
        x.insert(&key, &value);
    }
    x;
}
