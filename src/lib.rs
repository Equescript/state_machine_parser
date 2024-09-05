use std::hash::Hash;

mod bnf_rules_compiler;
mod error;
mod state_machine;
pub use bnf_rules_compiler::compile_bnf_rules;
pub use state_machine::{State, StateAction, StateMachineParser, debug_print_match_record};
pub use bnf_rules_compiler::StateManager;

pub trait TokenType: Clone + Eq + Hash {}

pub trait Token<TT> where TT: TokenType {
    fn token_type(&self) -> &TT;
}


mod tests {
    // use crate::compile_rules;

    use std::collections::HashMap;

    use crate::{compile_bnf_rules, debug_print_match_record, StateMachineParser, Token, TokenType};

    #[test]
    fn test_bnf_rules_compiler() {
        std::env::set_var("RUST_BACKTRACE", "1");
        const RULE: &str = "
        MultiplicationExpression = Number {(OperatorMul | OperatorDiv) Number};
        Expression               = MultiplicationExpression {(OperatorAdd | OperatorSub) MultiplicationExpression};
        ";
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum NumericExpressionTokenType {
            Number,
            OperatorAdd,
            OperatorSub,
            OperatorMul,
            OperatorDiv,
        }
        impl TokenType for NumericExpressionTokenType {}
        static mut TOKEN_TYPE_CONVERTER: Option<HashMap<Vec<char>, NumericExpressionTokenType>> = None;
        unsafe {
            TOKEN_TYPE_CONVERTER = Some(HashMap::from([
                ("Number".chars().collect::<Vec<char>>(), NumericExpressionTokenType::Number),
                ("OperatorAdd".chars().collect::<Vec<char>>(), NumericExpressionTokenType::OperatorAdd),
                ("OperatorSub".chars().collect::<Vec<char>>(), NumericExpressionTokenType::OperatorSub),
                ("OperatorMul".chars().collect::<Vec<char>>(), NumericExpressionTokenType::OperatorMul),
                ("OperatorDiv".chars().collect::<Vec<char>>(), NumericExpressionTokenType::OperatorDiv),
            ]));
        }
        impl TryFrom<Vec<char>> for NumericExpressionTokenType {
            type Error = ();
            fn try_from(value: Vec<char>) -> Result<Self, Self::Error> {
                match unsafe{TOKEN_TYPE_CONVERTER.as_ref().unwrap()}.get(&value) {
                    Some(t) => Ok(t.clone()),
                    None => Err(())
                }
            }
        }
        #[derive(Debug)]
        struct NumericExpressionToken {
            token_type: NumericExpressionTokenType,
            value: usize,
        }
        impl NumericExpressionToken {
            fn number(number: usize) -> Self {
                Self { token_type: NumericExpressionTokenType::Number, value: number }
            }
            fn operator(operator: NumericExpressionTokenType) -> Self {
                Self { token_type: operator, value: 0 }
            }
        }
        impl std::fmt::Display for NumericExpressionToken {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        impl Token<NumericExpressionTokenType> for NumericExpressionToken {
            fn token_type(&self) -> &NumericExpressionTokenType {
                &self.token_type
            }
        }
        let state_manager = compile_bnf_rules(RULE).unwrap();
        let expression = vec![
            NumericExpressionToken::number(1),
            NumericExpressionToken::operator(NumericExpressionTokenType::OperatorMul),
            NumericExpressionToken::number(2),
            NumericExpressionToken::operator(NumericExpressionTokenType::OperatorAdd),
            NumericExpressionToken::number(3),
            NumericExpressionToken::operator(NumericExpressionTokenType::OperatorMul),
            NumericExpressionToken::number(4),
            ];
        let start_rule = *state_manager.rule_ids.get(&"Expression".chars().collect::<Vec<char>>()).unwrap();
        let match_records = match StateMachineParser::new(&state_manager).parse(&expression, start_rule) {
            Ok(m) => m,
            Err(e) => {
                println!("{}", e);
                panic!();
            },
        };
        debug_print_match_record(&expression, &match_records, &state_manager.rule_names);
    }
}