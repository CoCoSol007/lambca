use std::ops::Range;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\n]*")]
pub enum TokenType {
    #[token(".")]
    Dot,

    #[token("\\")]
    Lambda,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("let")]
    Let,

    #[token("eval")]
    Eval,

    #[token("=")]
    Equals,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("\n")]
    NewLine,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Range<usize>,
}
