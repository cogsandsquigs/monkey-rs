use super::{precedence::Precedence, Parser};
use crate::{
    ast::{expression::Expression, statement::Statement},
    token::TokenType,
};

/// Describes a prefix parse function, which parses an operator that comes before a literal value/grouped expression.
pub(crate) type PrefixParseFn = fn(&mut Parser) -> Result<Statement, ()>;

/// Describes an infix parse function, which parses an operator that is in between some literal values/grouped expressions.
pub(crate) type InfixParseFn = fn(&mut Parser, Expression) -> Result<Statement, ()>;

impl Parser {
    /// Regesters a prefix function for a given token type.
    fn register_prefix(&mut self, token_type: TokenType, prefix_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, prefix_fn);
    }

    /// Regesters an infix function for a given token type.
    fn register_infix(&mut self, token_type: TokenType, infix_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, infix_fn);
    }

    /// Parses an expression from the input.
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ()> {
        todo!()
    }
}
