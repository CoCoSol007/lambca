//! All programs related to the lexer of the lambda calculus language.

use std::fmt::Display;
use std::ops::Range;

use logos::Logos;

/// Tokens types for the lambda calculus language.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\n]*")]
pub enum TokenType {
    /// The dot character `.`
    #[token(".")]
    Dot,

    /// The lambda character `\`
    #[token("\\")]
    Lambda,

    /// The left parenthesis `(`
    #[token("(")]
    LParen,

    /// The right parenthesis `)`
    #[token(")")]
    RParen,

    /// The `let` keyword
    #[token("let")]
    Let,

    /// The `eval` keyword
    #[token("eval")]
    Eval,

    /// The equals sign `=`
    #[token("=")]
    Equals,

    /// An identifier (variable name)
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    /// A new line character
    #[token("\n")]
    NewLine,
}

/// A token with its type and span in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of the token
    pub token_type: TokenType,

    /// The span (start and end indices) of the token in the source code
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
