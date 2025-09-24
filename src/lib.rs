pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm {
    Variable(String),
    LambdaAbstraction(String, Box<LambdaTerm>),
    Application(Box<LambdaTerm>, Box<LambdaTerm>),
}

impl LambdaTerm {
    pub fn beta_reduction(self) -> Self {
        let mut current = self;
        let mut reducted = current.clone().apply_beta_reduction(None, None);
        while current != reducted {
            current = reducted;
            reducted = current.clone().apply_beta_reduction(None, None);
        }

        reducted
    }

    fn apply_beta_reduction(
        self,
        variable: Option<String>,
        lambda_term: Option<LambdaTerm>,
    ) -> Self {
        match self {
            LambdaTerm::Variable(v) => {
                if lambda_term.is_some() {
                    let lambda = lambda_term.unwrap();
                    if Some(v.clone()) == variable {
                        return lambda;
                    }
                    return LambdaTerm::Application(LambdaTerm::Variable(v).into(), lambda.into());
                } else {
                    return LambdaTerm::Variable(v)
                }
            }
            LambdaTerm::LambdaAbstraction(s, a) => {
                if let Some(lambda_term) = lambda_term {
                    return a.clone().apply_beta_reduction(Some(s), Some(lambda_term));
                } else {
                    return LambdaTerm::LambdaAbstraction(s,a);
                }
            },
            LambdaTerm::Application(a, b) => {
                if lambda_term.is_none() {
                    return a.clone().apply_beta_reduction(None, Some(*b))
                } else {
                    return LambdaTerm::Application(a,b);
                }
            }
        }
    }
}
