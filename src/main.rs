//! Main program for the lambda calculus interpreter.

use std::collections::HashMap;
use std::process::exit;
use std::sync::RwLock;
use std::{env, fs};

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;
use chumsky::error::Rich;
use lambda::lexer::TokenType;
use logos::Logos;

#[tokio::main]
async fn main() {
    let cli_ok = match cli() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let CliResult::Text(text, path) = cli_ok else {
        println!("Usage: lambda <file_path>");
        exit(0);
    };

    let save_lambda_term = RwLock::new(HashMap::new());

    let mut tokens = Vec::new();
    let mut tokens_type = Vec::new();

    for (result_token_type, span) in TokenType::lexer(&text).spanned() {
        let Ok(token_type) = result_token_type else {
            continue;
        };
        let token = lambda::lexer::Token {
            token_type: token_type.clone(),
            span,
        };
        tokens_type.push(token_type);
        tokens.push(token);
    }

    let parser = lambda::parser::parser();
    let result = parser.parse(tokens_type.as_slice());

    match result.into_result() {
        Ok(instructions) => {
            Report::build(
                ReportKind::Custom("Info", ariadne::Color::Green),
                (&path, 0..text.len()),
            )
            .with_message(format!("Successfully parsed '{}'", path))
            .finish()
            .print((&path, Source::from(&text)))
            .unwrap();

            for instruction in instructions {
                instruction.compute(&save_lambda_term).await;
            }
        }
        Err(errors) => handle_error(errors.clone(), &path, &text, &tokens),
    };
}

/// Handle parser errors by reporting them with ariadne and exiting the
/// program.
fn handle_error(
    errors: Vec<Rich<TokenType>>,
    file_path: &str,
    source: &str,
    tokens: &Vec<lambda::lexer::Token>,
) {
    for e in errors {
        let span_token_type: std::ops::Range<usize> = e.span().into_iter();
        let span: std::ops::Range<usize> = tokens[span_token_type.start].span.clone();
        Report::build(ReportKind::Error, (file_path, span.clone()))
            .with_message("Parser Error")
            .with_label(Label::new((file_path, span)).with_message(format!(
                "expected {}",
                e.expected().map(|f| f.to_string()).collect::<Vec<_>>().join(", ")
            )))
            .finish()
            .print((file_path, Source::from(source)))
            .unwrap();
    }
    exit(1);
}

/// The result of the command line interface parsing.
enum CliResult {
    /// The text to parse and its file path.
    Text(String, String),

    /// Display the help message.
    Help,
}

/// Parse the command line arguments and return the file path to parse or an
/// error message.
fn cli() -> Result<CliResult, String> {
    let mut args = env::args();
    let Some(first) = args.nth(1) else {
        return Err("No arguments provided. \nUsage: lambda <file_path>".to_owned());
    };

    if first == "-h" || first == "--help" {
        return Ok(CliResult::Help);
    } else {
        let file_path = first;
        let text = fs::read_to_string(&file_path)
            .map_err(|_| "Could not read file \nUsage: lambda <file_path>".to_owned())?;
        return Ok(CliResult::Text(text, file_path));
    }
}
