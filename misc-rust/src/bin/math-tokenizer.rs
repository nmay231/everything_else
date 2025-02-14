// TODO: Potentially unclear example. I am just committing this, because I'm tired of so many leftover git stashes

struct TokenStream {
    input: Box<dyn Iterator<Item = char>>,
    index: usize,
}

enum Token {
    Int(i32),
    Sum,
    InvalidInput,
}

impl TokenStream {
    fn from(input: String) -> Self {
        Self {
            input: Box::new(input.chars()),
            index: 0,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
        // let str = self.input.get(self.index..self.index + 1)?;

        // // self.index += 1;
        // match str {
        //     _ if str.trim() == "" => None,
        //     _ => None,
        // }
    }
}

enum Node {
    Int(i32),
    Sum(Box<Node>, Box<Node>),
}

fn parse(mut stream: TokenStream) {
    for token in stream {}
}

fn main() {
    let input = "1+2";
    let mut stream = TokenStream::from(input.to_string());
    let tree = parse(stream);
    // for node in tree.walk() {
    //     println!("{}", node);
    // }
}
