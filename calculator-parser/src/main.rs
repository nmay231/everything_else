#[derive(Debug)]
pub(crate) enum Token<'parse> {
    Number(&'parse str),
    Plus,   // +
    Minus,  // -
    Times,  // *
    Divide, // /
    Power,  // ^
    LParen, // (
    RParen, // )
    Unknown(&'parse str),
}

#[derive(Debug, Copy, Clone)]
enum UnknownToken {
    Number,
    Other,
}

impl UnknownToken {
    fn to_known<'parse>(&self, text: &'parse str) -> Token<'parse> {
        match self {
            UnknownToken::Number => Token::Number(text),
            UnknownToken::Other => Token::Unknown(text),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TokenList<'parse> {
    text: &'parse str,
    tokens: Vec<Token<'parse>>,
}

impl<'parse> TokenList<'parse> {
    fn new(text: &'parse str) -> Self {
        let mut tokens = vec![];

        let mut unknown = None;
        let mut iter = text.chars().enumerate().peekable();
        while let Some((i, token)) = iter.next() {
            let mut is_partial_token = false;
            match token {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Times),
                '/' => tokens.push(Token::Divide),
                '^' => tokens.push(Token::Power),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ' ' | '\t' | '\n' | '\r' => (),
                c => {
                    unknown = match (c.is_ascii_digit(), unknown) {
                        (true, None) => Some((i, UnknownToken::Number)),
                        (false, None) => Some((i, UnknownToken::Other)),

                        (true, Some((start, token_type))) => Some((start, token_type)),
                        (false, Some((start, _))) => Some((start, UnknownToken::Other)),
                    };
                    is_partial_token = true;
                }
            }

            if let Some((start, token_type)) = unknown {
                if !is_partial_token {
                    tokens.push(token_type.to_known(&text[start..i]));
                    unknown = None;
                } else if iter.peek().is_none() {
                    tokens.push(token_type.to_known(&text[start..]));
                    unknown = None;
                }
            }
        }

        Self { text, tokens }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

fn main() {
    println!("{:?}", TokenList::new("1 + 2 * 3 / unknown"));
}
