use logos::{Lexer, Logos};
use std::fmt::Debug;

pub trait TypeEq {
    fn type_eq(&self, other: &Self) -> bool;
}

pub struct Tokenizer<'a, Token>
where
    Token: Logos<'a> + TypeEq + Clone + Debug,
{
    lexer: Lexer<'a, Token>,

    current: Option<Token>,
    index: usize,
    len: usize,

    next: Option<Token>,
    next_index: usize,
    next_len: usize,

    ignore: Vec<Token>,
}

impl<'a, Token> Tokenizer<'a, Token>
where
    Token: Logos<'a> + TypeEq + Clone + Debug,
{
    /// this creates a new Tokenizer
    ///
    /// ```rust
    /// use logos::Logos;
    /// use tokenizer::{Tokenizer, TypeEq};
    ///
    /// fn main() {
    ///     let code = "aa aaaa aa a";
    ///     let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Space]);
    /// }
    ///
    /// #[derive(Logos, Debug, Clone, PartialEq)]
    /// enum Token {
    ///     #[regex(r"[a]+", |lex| lex.slice().parse())]
    ///     A(String),
    ///     #[token(" ")]
    ///     Space,
    ///     #[error]
    ///     Unknown,
    /// }
    ///
    /// impl TypeEq for Token {
    ///     fn type_eq(&self, other: &Self) -> bool {
    ///         match (self, other) {
    ///             (Token::A(_), Token::A(_)) => true,
    ///             (Token::Space, Token::Space) => true,
    ///             _ => false,
    ///         }
    ///     }
    /// }
    /// ```
    pub fn new(mut lexer: Lexer<'a, Token>, ignore: Vec<Token>) -> Self {
        let current = None;
        let index = 0;
        let len = 0;

        let mut next = lexer.next();
        let mut next_index = lexer.span().start;
        let mut next_len = lexer.span().len();
        loop {
            if let Some(token) = next.clone() {
                let mut found = false;
                for t in ignore.iter() {
                    if token.type_eq(t) {
                        next = lexer.next();
                        next_index = lexer.span().start;
                        next_len = lexer.span().len();
                        found = true;
                        break;
                    }
                }

                if !found {
                    break;
                }
            } else {
                break;
            }
        }
        Self {
            lexer,
            current,
            next,
            index,
            len,
            next_index,
            next_len,
            ignore,
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.next.clone()
    }

    pub fn current(&self) -> Option<Token> {
        self.current.clone()
    }

    pub fn error(&self, msg: &str) -> Error {
        Error::new(Some(self.index), Some(self.len), msg.to_string())
    }

    pub fn expect(&mut self, token: Token) -> Result<Token, Error> {
        if let Some(got) = self.current() {
            self.next();
            if token.type_eq(&got) {
                Ok(got)
            } else {
                Err(self.error(&format!("expect token {:?} but got {:?}", token, got)))
            }
        } else {
            Err(self.error(&format!(
                "expect token {:?} but got {}",
                token, "end of file"
            )))
        }
    }

    fn can_ignore(&self, token: &Token) -> bool {
        for t in self.ignore.clone() {
            if t.type_eq(token) {
                return true;
            }
        }
        return false;
    }

    pub fn expect_multi(&mut self, tokens: Vec<Token>) -> Result<Token, Error> {
        if let Some(token) = self.current() {
            self.next();
            for t in tokens.iter() {
                if token.type_eq(t) {
                    return Ok(token);
                }
            }
            return Err(self.error(&format!("expect tokens {:?} but got {:?}", tokens, token)));
        } else {
            Err(self.error(&format!(
                "expect tokens {:?} but got {}",
                tokens, "end of file"
            )))
        }
    }

    pub fn is(&self, token: Token) -> bool {
        if let Some(got) = self.current.clone() {
            return got.type_eq(&token);
        }
        return false;
    }

    pub fn is_multi(&self, tokens: Vec<Token>) -> bool {
        if let Some(got) = self.current.clone() {
            for t in tokens {
                if got.type_eq(&t) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn next_is(&self, token: Token) -> bool {
        if let Some(got) = self.next.clone() {
            return got.type_eq(&token);
        }
        return false;
    }

    pub fn next_is_multi(&self, tokens: Vec<Token>) -> bool {
        if let Some(got) = self.next.clone() {
            for t in tokens {
                if got.type_eq(&t) {
                    return true;
                }
            }
        }
        return false;
    }
}

impl<'a, Token> Iterator for Tokenizer<'a, Token>
where
    Token: Logos<'a> + TypeEq + Clone + Debug,
{
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current = self.next.clone();
            self.index = self.next_index;
            self.len = self.next_len;

            self.next = self.lexer.next();
            self.next_index = self.lexer.span().start;
            self.next_len = self.lexer.span().len();

            if let Some(token) = self.current.clone() {
                if !self.can_ignore(&token) {
                    while let Some(token) = self.next.clone() {
                        if self.can_ignore(&token) {
                            self.next = self.lexer.next();
                            self.next_index = self.lexer.span().start;
                            self.next_len = self.lexer.span().len();
                        } else {
                            break;
                        }
                    }
                    return self.current.clone();
                }
            } else {
                return self.current.clone();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    index: Option<usize>,
    len: Option<usize>,
    msg: String,
}

impl Error {
    pub fn new(index: Option<usize>, len: Option<usize>, msg: String) -> Self {
        Self { index, len, msg }
    }
}
