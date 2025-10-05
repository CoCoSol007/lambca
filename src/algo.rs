//! All programs related to the lambda calculus language.

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::RwLock;

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
    /// Implements alpha conversion to avoid variable capture.
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
                } else if replacement.free_variables().contains(&param) {
                    let all_vars =
                        LambdaTerm::LambdaAbstraction(param.clone(), body.clone()).all_variables();
                    let fresh_param = Self::generate_fresh_variable(&param, &all_vars, replacement);
                    let renamed_body =
                        body.substitute(&param, &LambdaTerm::Variable(fresh_param.clone()));
                    LambdaTerm::LambdaAbstraction(
                        fresh_param,
                        Box::new(renamed_body.substitute(var, replacement)),
                    )
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

    /// Get all free variables in this lambda term.
    fn free_variables(&self) -> std::collections::HashSet<String> {
        use std::collections::HashSet;

        match self {
            LambdaTerm::Variable(v) => {
                let mut set = HashSet::new();
                set.insert(v.clone());
                set
            }
            LambdaTerm::LambdaAbstraction(param, body) => {
                let mut free_vars = body.free_variables();
                free_vars.remove(param);
                free_vars
            }
            LambdaTerm::Application(func, arg) => {
                let mut free_vars = func.free_variables();
                free_vars.extend(arg.free_variables());
                free_vars
            }
        }
    }

    /// Generate a fresh variable name that doesn't conflict with any variables
    /// in the current context or the replacement.
    fn generate_fresh_variable(
        base_name: &str,
        current_vars: &std::collections::HashSet<String>,
        replacement: &LambdaTerm,
    ) -> String {
        let mut used_vars = current_vars.clone();
        used_vars.extend(replacement.all_variables());

        let mut candidate = base_name.to_string();
        let mut counter = 0;

        while used_vars.contains(&candidate) {
            counter += 1;
            candidate = format!("{}{}", base_name, counter);
        }

        candidate
    }

    /// Get all variables (both free and bound) in this lambda term.
    fn all_variables(&self) -> std::collections::HashSet<String> {
        match self {
            LambdaTerm::Variable(v) => {
                let mut set = std::collections::HashSet::new();
                set.insert(v.clone());
                set
            }
            LambdaTerm::LambdaAbstraction(param, body) => {
                let mut vars = body.all_variables();
                vars.insert(param.clone());
                vars
            }
            LambdaTerm::Application(func, arg) => {
                let mut vars = func.all_variables();
                vars.extend(arg.all_variables());
                vars
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_variables() {
        // Variable: x
        let term = LambdaTerm::Variable("x".to_string());
        let free_vars = term.free_variables();
        assert!(free_vars.contains("x"));
        assert_eq!(free_vars.len(), 1);

        // Lambda abstraction: λx.x (no free variables)
        let term = LambdaTerm::LambdaAbstraction(
            "x".to_string(),
            Box::new(LambdaTerm::Variable("x".to_string())),
        );
        let free_vars = term.free_variables();
        assert_eq!(free_vars.len(), 0);

        // Lambda abstraction: λx.y (y is free)
        let term = LambdaTerm::LambdaAbstraction(
            "x".to_string(),
            Box::new(LambdaTerm::Variable("y".to_string())),
        );
        let free_vars = term.free_variables();
        assert!(free_vars.contains("y"));
        assert!(!free_vars.contains("x"));
        assert_eq!(free_vars.len(), 1);
    }

    #[test]
    fn test_alpha_conversion_simple() {
        // Test case: (λx.x) should substitute without alpha conversion
        let term = LambdaTerm::LambdaAbstraction(
            "x".to_string(),
            Box::new(LambdaTerm::Variable("x".to_string())),
        );
        let replacement = LambdaTerm::Variable("y".to_string());
        let result = term.substitute("z", &replacement);

        // Should remain unchanged since z is not bound in the term
        match result {
            LambdaTerm::LambdaAbstraction(param, _) => {
                assert_eq!(param, "x");
            }
            _ => panic!("Expected lambda abstraction"),
        }
    }

    #[test]
    fn test_alpha_conversion_with_capture() {
        // Test case: λy.(x y) where we substitute x with λz.y
        // This should trigger alpha conversion since y would be captured
        let term = LambdaTerm::LambdaAbstraction(
            "y".to_string(),
            Box::new(LambdaTerm::Application(
                Box::new(LambdaTerm::Variable("x".to_string())),
                Box::new(LambdaTerm::Variable("y".to_string())),
            )),
        );

        let replacement = LambdaTerm::LambdaAbstraction(
            "z".to_string(),
            Box::new(LambdaTerm::Variable("y".to_string())),
        );

        let result = term.substitute("x", &replacement);

        // The parameter should be renamed to avoid capturing the free 'y' in
        // replacement
        match result {
            LambdaTerm::LambdaAbstraction(param, body) => {
                assert_ne!(param, "y"); // Should be renamed
                assert!(param.starts_with("y")); // Should be based on "y"

                // Verify the body has the correct structure
                match body.as_ref() {
                    LambdaTerm::Application(func, arg) => {
                        // Function should be the replacement (λz.y)
                        match func.as_ref() {
                            LambdaTerm::LambdaAbstraction(z_param, z_body) => {
                                assert_eq!(z_param, "z");
                                match z_body.as_ref() {
                                    LambdaTerm::Variable(var) => assert_eq!(var, "y"),
                                    _ => panic!("Expected variable y in replacement body"),
                                }
                            }
                            _ => panic!("Expected lambda abstraction after substitution"),
                        }

                        // Argument should be the renamed parameter
                        match arg.as_ref() {
                            LambdaTerm::Variable(var) => assert_eq!(var, &param),
                            _ => panic!("Expected variable with renamed parameter"),
                        }
                    }
                    _ => panic!("Expected application in body"),
                }
            }
            _ => panic!("Expected lambda abstraction"),
        }
    }

    #[test]
    fn test_generate_fresh_variable() {
        let mut used_vars = std::collections::HashSet::new();
        used_vars.insert("x".to_string());
        used_vars.insert("x1".to_string());

        let replacement = LambdaTerm::Variable("x2".to_string());

        let fresh = LambdaTerm::generate_fresh_variable("x", &used_vars, &replacement);
        assert_eq!(fresh, "x3");
    }
}
