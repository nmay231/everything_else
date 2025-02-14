use std::f128::consts::E;

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

    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn consume(self) -> Vec<Token<'parse>> {
        return self.tokens;
    }
}

#[derive(Debug)]
enum Expression<'parse> {
    U32(u32),
    Add(&'parse Expression<'parse>, &'parse Expression<'parse>),
    Subtract(&'parse Expression<'parse>, &'parse Expression<'parse>),
    Multiply(&'parse Expression<'parse>, &'parse Expression<'parse>),
    Divide(&'parse Expression<'parse>, &'parse Expression<'parse>),
    Power(&'parse Expression<'parse>, &'parse Expression<'parse>),
}

#[derive(Debug, Default)]
struct PartialExpression<'parse> {
    operator: Option<Token<'parse>>,
    lhs: Option<&'parse Expression<'parse>>,
    rhs: Option<&'parse Expression<'parse>>,
}

#[derive(Debug)]
struct ParsedTree<'parse>(Expression<'parse>);

#[derive(Debug)]
pub(crate) struct Parsed<'parse> {
    text: &'parse str,
    expressions: Vec<Expression<'parse>>,
    tree: ParsedTree<'parse>,
}

#[derive(Debug)]
enum ParserState<'parse> {
    AfterOperator {
        operator: Token<'parse>,
        lhs: Option<&'parse Expression<'parse>>,
    },
    AfterNumber {},
}

struct Parsing<'parse> {
    input_tokens: Vec<Token<'parse>>,
    stack: Vec<Token<'parse>>,
    partial_thingy: Vec<PartialParsedThingy<'parse>>,
    expressions: Vec<Expression<'parse>>,
}

enum NakedOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}

enum PartialParsedThingy<'parse> {
    Number(u32),
    NumberOperator(&'parse Expression<'parse>, NakedOperator),
    Triple(&'parse Expression<'parse>, NakedOperator, &'parse PartialParsedThingy<'parse>),
}

impl<'parse> Parsing<'parse> {
    fn expect_number_or_lparen(&mut self) -> anyhow::Result<()> {
        while let Some(Token::LParen) = self.input_tokens.pop() {
            self.stack.push(Token::LParen);
            self.expect_number_or_lparen()?;
        }
        match self.input_tokens.pop() {
            Some(Token::Number(text)) => Ok(text),
            Some(token) => Err(anyhow::anyhow!("Expected number, got {:?}", token)),
            None => Err(anyhow::anyhow!("Expected something, got nothing")),
        }
    }
}

impl<'parse> Parsed<'parse> {
    fn new(tokens: TokenList<'parse>) -> anyhow::Result<Self> {
        let mut stack = vec![];
        let mut expressions = vec![];
        let tokens = tokens.consume().into_iter().rev().collect::<Vec<_>>();

        while let Some(Token::LParen) = tokens.pop() {
            stack.push(Token::LParen);
        }

        match tokens.pop() {
            Some(token @ Token::Number(_)) => {
                stack.push(token);
            }
            _ => return Err(anyhow::anyhow!("Expected at least one number")),
        }

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => return Err(anyhow::anyhow!("Number after number")),
                token@ (Token::Times | Token::Divide | Token::Power) => {
                    ;
            }
        }
    }

    fn new_dont_know(tokens: TokenList<'parse>) -> anyhow::Result<Self> {
        let mut expressions = vec![];
        let tokens = tokens.consume();
        let mut root = PartialExpression::default();
        for token in &tokens {
            let x = match token {
                // TODO: Handle numbers larger than u32
                Token::Number(text) => Some(Expression::U32(text.parse()?)),
                Token::Plus => todo!(),
                Token::Minus => todo!(),
                Token::Times => todo!(),
                Token::Divide => todo!(),
                Token::Power => todo!(),
                Token::LParen => todo!(),
                Token::RParen => todo!(),
                Token::Unknown(_) => panic!("Unknown token"),
            };
        }
        let mut tree = ParsedTree(Expression::Number("0"));
    }
}

fn main() {
    let tokens = TokenList::new("1 + 2 * 3 / unknown");
    let parsed = Parsed::new(tokens);
    println!("{:?}", parsed);
}
