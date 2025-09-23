pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaTerm<'src> {
    Variable(&'src str),
    LambdaAbstraction(&'src str, Box<LambdaTerm<'src>>),
    Application(Box<LambdaTerm<'src>>, Box<LambdaTerm<'src>>),
}

impl<'str> LambdaTerm<'str> {
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
                if let Some(lambda_term) = lambda_term
                    && Some(v) == variable.as_deref()
                {
                    return lambda_term;
                } else {
                    return self;
                }
            }
            LambdaTerm::LambdaAbstraction(_, lambda_term) => todo!(),
            LambdaTerm::Application(a, b) => {
                if lambda_term.is_none() {
                    a.apply_beta_reduction(None, Some(*b))
                } else {
                }
            }
        }
    }
}
