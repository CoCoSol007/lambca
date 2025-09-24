//! TODO

use chumsky::prelude::{just, recursive};
use chumsky::{Parser, text};

use crate::LambdaTerm;

pub fn parser<'src>() -> impl Parser<'src, &'src str, LambdaTerm> {
    let ident = text::ascii::ident().padded();
    recursive(
        |expr: chumsky::prelude::Recursive<dyn Parser<'_, &'src str, LambdaTerm>>| {
            let op = |c| just(c).padded();

            let variable = ident.clone().map(|s: &str| LambdaTerm::Variable(s.to_owned()));

            let lambda_abs = op('\\')
                .ignore_then(ident)
                .then_ignore(just("."))
                .then(expr.clone())
                .map(|(param, body)| LambdaTerm::LambdaAbstraction(param.to_owned(), body.into()));

            let application = just("(")
                .ignore_then(expr.clone())
                .then_ignore(just(","))
                .then(expr)
                .then_ignore(just(")"))
                .map(|(func, arg)| LambdaTerm::Application(func.into(), arg.into()));

            lambda_abs.or(application).or(variable)
        },
    )
}
