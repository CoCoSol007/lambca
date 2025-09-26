//! A parser for a simple lambda calculus language using the `chumsky` crate.

use crate::lexer::TokenType;
use crate::{Instruction, LambdaTerm};

use chumsky::error::Rich;
use chumsky::prelude::*;

pub fn parser<'src>()
-> impl Parser<'src, &'src [TokenType], Vec<Instruction>, extra::Err<Rich<'src, TokenType>>> + Clone
{
    let ident = select! {
        TokenType::Identifier(name) => name.to_owned(),
    };

    let lambda_term = recursive(|expr| {
        let variable = ident
            .labelled("variable identifier")
            .map(LambdaTerm::Variable)
            .labelled("variable");

        let lambda_abs = just(TokenType::Lambda)
            .labelled("lambda")
            .ignore_then(ident)
            .labelled("parameter")
            .then_ignore(just(TokenType::Dot).labelled("dot"))
            .then(expr.clone())
            .map(|(param, body)| LambdaTerm::LambdaAbstraction(param, Box::new(body)))
            .labelled("lambda abstraction");

        let application = just(TokenType::LParen)
            .ignore_then(expr.clone())
            .then(expr)
            .then_ignore(just(TokenType::RParen))
            .map(|(func, arg)| LambdaTerm::Application(func.into(), arg.into()))
            .labelled("application");

        choice((lambda_abs, application, variable))
    })
    .labelled("lambda term");

    let let_term = just(TokenType::Let)
        .ignore_then(ident)
        .then_ignore(just(TokenType::Equals))
        .then(lambda_term.clone())
        .map(|(name, body)| Instruction::Let {
            name,
            lambda_term: body,
        })
        .labelled("let binding");

    let eval_term = just(TokenType::Eval)
        .ignore_then(lambda_term.clone())
        .map(|body| Instruction::Eval { lambda_term: body })
        .labelled("eval instruction");

    choice((let_term, eval_term))
        .separated_by(just(TokenType::NewLine).repeated().at_least(1))
        .allow_leading()
        .allow_trailing()
        .collect()
}
