use std::{collections::HashMap, fmt::Error, fs, sync::RwLock};

use chumsky::Parser;
use lambda::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = <Cli as clap::Parser>::parse();

    let file_content_result = match cli {
        Cli::RunFile { path } => fs::read_to_string(path),
        Cli::Run { expr } => Ok(expr),
    };

    if let Err(error) = file_content_result {
        println!("Error: {}", error);
        return Err(());
    }

    let text = file_content_result.unwrap();

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
            println!("Parse errors:");
            for error in errors {
                println!("  {:?}", error);
            }
        }
    };

    Ok(())
}
