use super::{precedence::Precedence, Parser};
use crate::{
    ast::{
        expression::Identifier,
        statement::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
    },
    token::TokenType,
};

impl Parser {
    /// The `parse_statement` method parses a single statement from the input.
    /// TODO: This should return an actual error, not just `()`.
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.current_token.r#type {
            TokenType::Let => Ok(Statement::LetStatement(self.parse_let_statement()?)),
            TokenType::Return => Ok(Statement::ReturnStatement(self.parse_return_statement()?)),
            _ => Ok(Statement::ExpressionStatement(
                self.parse_expression_statement()?,
            )),
        }
    }

    /// The `parse_let_statement` method parses a `let` statement from the input. Expects the current
    /// token to be a `TokenType::Let`.
    /// TODO: This should return an actual error, not just `()`.
    fn parse_let_statement(&mut self) -> Result<LetStatement, ()> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return Err(()); // TODO: ReturnStatement an actual error.
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        // Need to check for `TokenType::Assign` here.
        if !self.expect_peek(TokenType::Assign) {
            return Err(()); // TODO: ReturnStatement an actual error.
        }

        // TODO: We're skipping expressions until we get to the semicolon.
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(LetStatement {
            token,
            name,
            value: None,
        })
    }

    /// The `parse_return_statement` method parses a `return` statement from the input. Expects the
    /// current token to be a `TokenType::Return`.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ()> {
        let token = self.current_token.clone();

        self.next_token();

        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()
        }

        Ok(ReturnStatement { token, value: None })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, ()> {
        let token = self.current_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        // Parse the ending semicolon (if it exists).
        // TODO: Raise error or smth if it doesn't exist.
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(ExpressionStatement { token, expression })
    }
}
