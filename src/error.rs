use crate::bnf_rules_compiler::{RuleId, StateId};

#[derive(Debug)]
pub enum BNFCompileError {
    UnknownError,
    ParseError(ParseError),
    UndefinedTokenIdentifier,
    UndefinedRuleIdentifier,
    UnexpectedToken,
    UnexpectedAction,
}

impl std::fmt::Display for BNFCompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BNFCompileError {}

#[derive(Debug)]
pub enum ParseError {
    LeftRecursion(Vec<RuleId>),
    UnexpectedToken(usize),
    UnexpectedState(StateId),
    UnexpectedRule(RuleId),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}
