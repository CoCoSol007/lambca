use std::{fmt::Display, ops::Range};

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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Dot => write!(f, "dot (.)"),
            TokenType::Lambda => write!(f, "lambda (\\)"),
            TokenType::LParen => write!(f, "left parenthesis '('"),
            TokenType::RParen => write!(f, "right parenthesis ')'"),
            TokenType::Let => write!(f, "let"),
            TokenType::Eval => write!(f, "eval"),
            TokenType::Equals => write!(f, "equals"),
            TokenType::Identifier(name) => write!(f, "identifier ({})", name),
            TokenType::NewLine => write!(f, "newline"),
        }
    }
}
