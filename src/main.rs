use std::{collections::HashMap, env, fs, process::exit, sync::RwLock};

use chumsky::Parser;

#[tokio::main]
async fn main() {
    let cli_ok = match cli() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let CliOk::Text(text) = cli_ok else {
        println!("Usage: lambda <file_path> or lambda -e <expression_to_beta_reduce>");
        exit(0);
    };

    let save_lambda_term = RwLock::new(HashMap::new());

    let parser = lambda::parser::parser();
    let result = parser.parse(&text);

    match result.into_result() {
        Ok(instructions) => {
            for instruction in instructions {
                instruction.compute(&save_lambda_term).await;
            }
        }
        Err(errors) => {
            for error in errors {
                println!("Parse error: {:?}", error);
            }
        }
    };
}

pub enum CliOk {
    Text(String),
    Help,
}

fn cli() -> Result<CliOk, String> {
    let mut args = env::args();
    let Some(first) = args.nth(1) else {
        return Err(
            "No arguments provided. \nUsage: lambda <file_path> or lambda -e <expression_to_beta_reduce>".to_owned(),
        );
    };

    if first == "-h" || first == "--help" {
        return Ok(CliOk::Help);
    } else if first == "-e" {
        let Some(expression) = args.nth(0) else {
            return Err("No expression provided after -e flag. \nUsage: lambda -e <expression_to_beta_reduce> \nor lambda <file_path> in order to read from a file.".to_owned());
        };
        return Ok(CliOk::Text(format!("eval {}", expression)));
    } else {
        let file_path = first;
        let text = fs::read_to_string(&file_path).map_err(|_| "Could not read file \nUsage: lambda <file_path> or lambda -e <expression_to_beta_reduce>".to_owned())?;
        return Ok(CliOk::Text(text));
    }
}
