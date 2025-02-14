// TODO: Potentially unclear example. I am just committing this, because I'm tired of so many leftover git stashes

struct WrapString {
    inner: Box<String>,
}

impl WrapString {
    fn from(string: String) -> Self {
        Self {
            inner: Box::new(string),
        }
    }
}

struct WrapIterator {
    input: String,
    inner: Box<dyn Iterator<Item = char>>,
}

impl WrapIterator {
    fn from(input: String) -> Self {
        Self {
            input,
            inner: Box::new(input.chars()),
        }
    }
}

impl Iterator for WrapIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.to_digit(10)? as i32)
    }
}

// fn func<'a, 'b>(input: &'a str) -> Result<&'a str, &'b str> {
//     let a: String = input.chars().collect();
//     Ok(&a[..])
// }

fn main() {
    let unwrapped = "test".to_string();
    let wrapped = WrapIterator::from(unwrapped.to_owned());
    for x in wrapped {
        println!("{}", x);
    }

    let s = String::from("testing");
    let wrapped = WrapString::from(s);
    println!("{}", wrapped.inner);

    // println!("{}, {}", wrapped.inner, unwrapped);

    // let test = func("asdf").unwrap();
    // println!("{}", test);
}
