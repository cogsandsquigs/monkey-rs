use super::{
    errors::Error,
    precedence::{precedence_of, Precedence},
    Parser,
};
use crate::{
    ast::expressions::{
        Boolean, Expression, Identifier, IfExpression, InfixExpression, Integer, PrefixExpression,
    },
    token::TokenType,
};

/// Describes a prefix parse function, which parses an operator that comes before a literal value/grouped expression.
pub(crate) type PrefixParseFn = fn(&mut Parser) -> Result<Expression, ()>;

/// Describes an infix parse function, which parses an operator that is in between some literal values/grouped expressions.
pub(crate) type InfixParseFn = fn(&mut Parser, Expression) -> Result<Expression, ()>;

impl Parser {
    /// Parses an expression from the input, using the Pratt Parsing technique.
    /// See: https://en.wikipedia.org/wiki/Pratt_parser
    /// Expects the current token to be the first token of the expression, i.e. a literal value/grouped expression/identifier.
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ()> {
        let Some(prefix) = self.prefix_parse_fns.get(&self.current_token.r#type) else {
            self.errors.push(Error::new(
                format!("no prefix parse function for {} found", self.current_token.r#type),
            ));

            return Err(());
        };

        // Mutable because we may need to modify the left-hand side of the expression later on in the loop.
        let mut left = prefix(self)?;

        // If the next token is a semicolon, we've reached the end of the expression. Therefore, we can
        // return the left-hand side of the expression, and stop parsing.
        //
        // If we haven't reached the end of the expression, we need to check if the next precedence is greater
        // than the current precedence. If it is, that means that the next operator "binds higher" so to speak, and
        // we must parse that operator first before adding it onto the tree.
        //
        // Otherwise, if the next precedence is less than the current precedence, we can return the left-hand side
        // of the expression, because we have reached a point where the next operator "binds lower" so to speak, and
        // we need to parse this current expression first before adding it onto the tree.
        //
        // Note that currently, there is only left-associativity for operators, so we don't need to check for equal
        // precedence and right-associativity. Also, we don't need to necessarily check for a semicolon (as all other
        // tokens that aren't operators have the precedence `Lowest`, which is less than the precedence of all operators).
        // However, it makes the code a bit more readable, and it's a good idea to check for a semicolon anyway.
        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let Some(infix) = self.infix_parse_fns.get(&self.peek_token.r#type).copied() else {
                // If we don't have an infix parse function for the next token, we can return the left-hand side of
                // the expression, because the statement has ended, and we need to parse the next statement. The 
                // `parse_expression_statement` function will handle the remaining tokens.
                return Ok(left);
            };

            // Advance the token pointer.
            self.next_token();

            // Append the next expression to the current tree (`left`).
            left = infix(self, left)?;
        }

        // Returning `left_expr` here b/c we aren't parsing infixes (yet).
        Ok(left)
    }

    /// Parses an identifier from the input. Expects the current token to be an identifier.
    fn parse_identifier(&mut self) -> Result<Expression, ()> {
        Ok(Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    /// Parses an integer from the input. Expects the current token to be an integer.
    fn parse_integer(&mut self) -> Result<Expression, ()> {
        let token = self.current_token.clone();

        let value = match token.literal.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                self.errors.push(Error::new(format!(
                    "could not parse {} as integer",
                    token.literal
                )));

                return Err(());
            }
        };

        Ok(Expression::Integer(Integer { token, value }))
    }

    /// Parses a boolean from the input. Expects the current token to be a boolean.
    fn parse_boolean(&mut self) -> Result<Expression, ()> {
        Ok(Expression::Boolean(Boolean {
            token: self.current_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    /// Parses a prefix expression from the input. e.g. `!5` or `-15`. Expects the current token to be a prefix operator.
    fn parse_prefix(&mut self) -> Result<Expression, ()> {
        let token = self.current_token.clone();
        let operator = token.literal.clone();

        // Advance to the next token so we can parse the right-hand side of the expression.
        self.next_token();

        // The precedence here is `Prefix` b/c we're parsing a prefix expression, which binds
        // tighter than any other operator.
        let right = self.parse_expression(Precedence::Prefix)?;

        Ok(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }

    /// Parses an infix expression from the input. e.g. `5 + 5` or `5 * 5`. Expects the current token to be an infix operator.
    fn parse_infix(&mut self, left: Expression) -> Result<Expression, ()> {
        let token = self.current_token.clone();
        let operator = token.literal.clone();

        // Get the precedence of our current operator - we pass this into `parse_expression` so
        // that we can parse the right-hand side of the expression with the correct precedence/binding power,
        // giving us a correct AST with order of operations.
        let precedence = self.current_precedence();

        // Advance to the next token so we can parse the right-hand side of the expression.
        self.next_token();

        let right = self.parse_expression(precedence)?;

        Ok(Expression::Infix(InfixExpression {
            token,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }))
    }

    /// Parses a grouped expression from the input. e.g. `(5 + 5)`. Expects the current token to be a left parenthesis.
    fn parse_grouped(&mut self) -> Result<Expression, ()> {
        // Advance to the next token so we can parse the expression inside the parentheses.
        self.next_token();

        // Parse the expression inside the parentheses.
        let expr = self.parse_expression(Precedence::Lowest)?;

        // If the next token isn't a right parenthesis, we have an error.
        if !self.expect_peek(TokenType::RParen) {
            return Err(());
        }

        Ok(expr)
    }

    /// Parses an if expression from the input. e.g. `if (x < y) { x }`. Expects the current token to be an `if` keyword
    /// (TokenKind::If).
    fn parse_if(&mut self) -> Result<Expression, ()> {
        let token = self.current_token.clone();

        // If the next token isn't a left parenthesis, we have an error.
        if !self.expect_peek(TokenType::LParen) {
            return Err(());
        }

        // Advance to the next token so we can parse the condition expression.
        self.next_token();

        // Parse the condition expression.
        let condition = self.parse_expression(Precedence::Lowest)?;

        // If the next token isn't a right parenthesis, we have an error.
        if !self.expect_peek(TokenType::RParen) {
            return Err(());
        }

        // If the next token isn't a left brace, we have an error.
        if !self.expect_peek(TokenType::LBrace) {
            return Err(());
        }

        // Parse the consequence block.
        let consequence = self.parse_block_statement()?;

        Ok(Expression::If(IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative: None,
        }))
    }
}

/// Private, not-necessarily-parsing functions. However, they are integral to the parsing process.
impl Parser {
    /// Peeks at the next token's precedence value.
    fn peek_precedence(&self) -> Precedence {
        precedence_of(&self.peek_token.r#type)
    }

    /// Returns the current token's precedence value.
    fn current_precedence(&self) -> Precedence {
        precedence_of(&self.current_token.r#type)
    }

    /// Regesters a prefix function for a given token type.
    fn register_prefix(&mut self, token_type: TokenType, prefix_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, prefix_fn);
    }

    /// Regesters an infix function for a given token type.
    fn register_infix(&mut self, token_type: TokenType, infix_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, infix_fn);
    }

    /// Registers tokens with their respective parse functions.
    pub(crate) fn register_tokens(&mut self) {
        // Registering prefix tokens.
        self.register_prefix(TokenType::Ident, Parser::parse_identifier);
        self.register_prefix(TokenType::Int, Parser::parse_integer);
        self.register_prefix(TokenType::True, Parser::parse_boolean);
        self.register_prefix(TokenType::False, Parser::parse_boolean);
        self.register_prefix(TokenType::Bang, Parser::parse_prefix);
        self.register_prefix(TokenType::Minus, Parser::parse_prefix);
        self.register_prefix(TokenType::LParen, Parser::parse_grouped);
        self.register_prefix(TokenType::If, Parser::parse_if);

        // Registering infix tokens.
        self.register_infix(TokenType::Plus, Self::parse_infix);
        self.register_infix(TokenType::Minus, Self::parse_infix);
        self.register_infix(TokenType::Slash, Self::parse_infix);
        self.register_infix(TokenType::Star, Self::parse_infix);
        self.register_infix(TokenType::Eq, Self::parse_infix);
        self.register_infix(TokenType::NotEq, Self::parse_infix);
        self.register_infix(TokenType::Lt, Self::parse_infix);
        self.register_infix(TokenType::Gt, Self::parse_infix);
    }
}
