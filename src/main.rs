use std::fs;

use chumsky::Parser;
use lambda::cli::Cli;

fn main() {
    let cli = <Cli as clap::Parser>::parse();

    let file_content_result = match cli {
        Cli::RunFile { path } => fs::read_to_string(path),
        Cli::Run { expr } => Ok(expr),
    };

    if let Err(error) = file_content_result {
        println!("Error: {}", error);
        return;
    }

    let text = file_content_result.unwrap();

    let parser = lambda::parser::parser();
    let ast = parser.parse(&text).unwrap();
    println!("{}", ast.beta_reduction())
}
