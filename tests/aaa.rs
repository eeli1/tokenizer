use logos::Logos;
use tokenizer::{Tokenizer, TypeEq};

#[test]
fn main() {
    let code = "  aa aaaa aa a";
    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Space]);

    assert_eq!(tokenizer.current(), None);
    assert_eq!(tokenizer.peek(), Some(Token::A("aa".to_string())));
    assert_eq!(tokenizer.next(), Some(Token::A("aa".to_string())));

    assert_eq!(tokenizer.current(), Some(Token::A("aa".to_string())));
    assert_eq!(tokenizer.peek(), Some(Token::A("aaaa".to_string())));
    assert_eq!(tokenizer.next(), Some(Token::A("aaaa".to_string())));

    assert_eq!(tokenizer.next(), Some(Token::A("aa".to_string())));
    assert_eq!(tokenizer.current(), Some(Token::A("aa".to_string())));
    assert_eq!(tokenizer.peek(), Some(Token::A("a".to_string())));

    assert_eq!(tokenizer.next(), Some(Token::A("a".to_string())));
    assert_eq!(tokenizer.peek(), None);
    assert_eq!(tokenizer.current(), Some(Token::A("a".to_string())));

    assert_eq!(tokenizer.next(), None);
    assert_eq!(tokenizer.current(), None);
    assert_eq!(tokenizer.peek(), None);
}

#[derive(Logos, Debug, Clone, PartialEq)]
enum Token {
    #[regex(r"[a]+", |lex| lex.slice().parse())]
    A(String),
    #[token(" ")]
    Space,
    #[error]
    Unknown,
}

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::A(_), Token::A(_)) => true,
            (Token::Space, Token::Space) => true,
            _ => false,
        }
    }
}