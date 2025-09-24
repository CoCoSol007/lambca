//! TODO

use chumsky::prelude::{any, end, just, recursive};
use chumsky::text::newline;
use chumsky::{IterParser, Parser, text};

use crate::{Instruction, LambdaTerm};

pub fn parser<'src>() -> impl Parser<'src, &'src str, Vec<Instruction>> {
    let ident = text::ascii::ident().padded();
    let op = |c| just(c).padded();

    let lambda_term = recursive(
        |expr: chumsky::prelude::Recursive<dyn Parser<'_, &'src str, LambdaTerm>>| {
            let variable = ident
                .clone()
                .map(|s: &str| LambdaTerm::Variable(s.to_owned()));

            let lambda_abs = op("\\")
                .ignore_then(ident)
                .then_ignore(just("."))
                .then(expr.clone())
                .map(|(param, body)| LambdaTerm::LambdaAbstraction(param.to_owned(), body.into()));

            let application = just("(")
                .ignore_then(expr.clone())
                .then_ignore(op(","))
                .then(expr)
                .then_ignore(just(")"))
                .map(|(func, arg)| LambdaTerm::Application(func.into(), arg.into()));

            lambda_abs.or(application).or(variable)
        },
    );

    let let_term = op("let")
        .ignore_then(ident)
        .then_ignore(op("="))
        .then(lambda_term.clone())
        .map(|(name, body)| Instruction::Let {
            name: name.to_owned(),
            lambda_term: body,
        });

    let eval_term = op("eval")
        .ignore_then(lambda_term)
        .map(|body| Instruction::Eval { lambda_term: body });

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    (let_term.or(eval_term))
        .padded()
        .padded_by(comment.repeated())
        .repeated()
        .collect()
}
