use super::{errors::Error, precedence::Precedence, Parser};
use crate::{ast::expression::Expression, token::TokenType};

/// Describes a prefix parse function, which parses an operator that comes before a literal value/grouped expression.
pub(crate) type PrefixParseFn = fn(&mut Parser) -> Result<Expression, ()>;

/// Describes an infix parse function, which parses an operator that is in between some literal values/grouped expressions.
pub(crate) type InfixParseFn = fn(&mut Parser, Expression) -> Result<Expression, ()>;

impl Parser {
    /// Parses an expression from the input.
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ()> {
        let Some(prefix) = self.prefix_parse_fns.get(&self.current_token.r#type) else {
            self.errors.push(Error::new(
                format!("no prefix parse function for {} found", self.current_token.r#type),
            ));

            return Err(());
        };

        let left_expr = prefix(self)?;

        // Returning `left_expr` here b/c we aren't parsing infixes (yet).
        Ok(left_expr)
    }

    /// Regesters a prefix function for a given token type.
    fn register_prefix(&mut self, token_type: TokenType, prefix_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, prefix_fn);
    }

    /// Regesters an infix function for a given token type.
    fn register_infix(&mut self, token_type: TokenType, infix_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, infix_fn);
    }
}