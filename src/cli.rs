use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "lambda", about = "A DSL for lambda expresion.")]
pub enum Cli {
    /// Evaluate a lambda expression from a file.
    RunFile {
        /// Path to the file containing the lambda expression.
        path: PathBuf,
    },

    /// Evaluate a lambda expression provided as plain text.
    Run {
        /// lambda expression to parse and evaluate.
        expr: String,
    },
}
