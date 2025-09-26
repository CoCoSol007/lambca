//! All programs related to the lambda calculus language.

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::RwLock;

pub mod lexer;
pub mod parser;

/// A lambda calculus term.
#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm {
    /// A variable, represented by its name.
    Variable(String),

    /// A lambda abstraction, represented by its parameter and body. (\x.body)
    LambdaAbstraction(String, Box<LambdaTerm>),

    /// An application, represented by its function and argument. (func arg)
    Application(Box<LambdaTerm>, Box<LambdaTerm>),
}

impl LambdaTerm {
    /// Perform beta reduction on the lambda term until no more reductions can
    /// be made.
    pub fn beta_reduction(self, save_lambda_term: HashMap<String, LambdaTerm>) -> Self {
        let mut current = self;
        loop {
            let reduced = current
                .clone()
                .apply_beta_reduction(save_lambda_term.clone());
            if current == reduced {
                break;
            }
            current = reduced;
        }
        current
    }

    /// Apply a single step of beta reduction to the lambda term.
    fn apply_beta_reduction(self, save_lambda_term: HashMap<String, LambdaTerm>) -> Self {
        match self {
            LambdaTerm::Variable(v) => {
                if let Some(lambda) = save_lambda_term.get(&v) {
                    return lambda.clone();
                } else {
                    return LambdaTerm::Variable(v);
                }
            }
            LambdaTerm::LambdaAbstraction(param, body) => LambdaTerm::LambdaAbstraction(
                param,
                Box::new(body.apply_beta_reduction(save_lambda_term)),
            ),
            LambdaTerm::Application(func, arg) => {
                let func_reduced = func.apply_beta_reduction(save_lambda_term.clone());
                let arg_reduced = arg.apply_beta_reduction(save_lambda_term);

                match func_reduced {
                    LambdaTerm::LambdaAbstraction(param, body) => {
                        body.substitute(&param, &arg_reduced)
                    }
                    _ => LambdaTerm::Application(Box::new(func_reduced), Box::new(arg_reduced)),
                }
            }
        }
    }

    /// Substitute all occurrences of a variable with a given lambda term.
    /// TODO: Implement alpha conversion to avoid variable capture.
    fn substitute(self, var: &str, replacement: &LambdaTerm) -> Self {
        match self {
            LambdaTerm::Variable(v) => {
                if v == var {
                    replacement.clone()
                } else {
                    LambdaTerm::Variable(v)
                }
            }
            LambdaTerm::LambdaAbstraction(param, body) => {
                if param == var {
                    LambdaTerm::LambdaAbstraction(param, body)
                } else {
                    LambdaTerm::LambdaAbstraction(
                        param,
                        Box::new(body.substitute(var, replacement)),
                    )
                }
            }
            LambdaTerm::Application(func, arg) => LambdaTerm::Application(
                Box::new(func.substitute(var, replacement)),
                Box::new(arg.substitute(var, replacement)),
            ),
        }
    }
}

impl Display for LambdaTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambdaTerm::Variable(name) => write!(f, "{}", name),
            LambdaTerm::LambdaAbstraction(param, body) => {
                write!(f, "λ{}.{}", param, body)
            }
            LambdaTerm::Application(func, arg) => {
                write!(f, "({} {})", func, arg)
            }
        }
    }
}

/// An instruction in the lambda calculus language.
#[derive(Debug)]
pub enum Instruction {
    /// Bind a name to a lambda term.
    Let {
        /// The name to bind.
        name: String,

        /// The lambda term to bind to the name.
        lambda_term: LambdaTerm,
    },

    /// Evaluate the lambda term. (Evaluation is done via beta reduction and
    /// printing the result)
    Eval(LambdaTerm),
}

impl Instruction {
    /// Execute the instruction, modifying the provided hashmap of saved lambda
    /// terms as needed.
    /// For `Let` instructions, the hashmap is updated with the new binding.
    /// For `Eval` instructions, the lambda term is beta-reduced and printed to
    /// the console.
    pub async fn compute(self, save_lambda_term: &RwLock<HashMap<String, LambdaTerm>>) {
        match self {
            Instruction::Let { name, lambda_term } => {
                let mut write = save_lambda_term.write().unwrap();
                write.insert(name, lambda_term);
            }
            Instruction::Eval(lambda_term) => {
                let read = save_lambda_term.read().unwrap();
                println!("{}", lambda_term.beta_reduction(read.clone()));
            }
        }
    }
}
