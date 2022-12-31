use super::{precedence::Precedence, Parser};
use crate::{
    ast::{
        expressions::{BlockStatement, Identifier},
        statements::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
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
            return Err(()); // TODO: return an actual error.
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        // Need to check for `TokenType::Assign` here.
        if !self.expect_peek(TokenType::Assign) {
            return Err(()); // TODO: return an actual error.
        }

        // Advance, as the current token is an `Assign`.
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        // TODO: We're skipping expressions until we get to the semicolon.
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(LetStatement {
            token,
            name,
            value: Box::new(value),
        })
    }

    /// The `parse_return_statement` method parses a `return` statement from the input. Expects the
    /// current token to be a `TokenType::Return`.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ()> {
        let token = self.current_token.clone();

        self.next_token();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()
        }

        Ok(ReturnStatement {
            token,
            return_value: Box::new(return_value),
        })
    }

    /// The `parse_expression_statement` method parses an expression statement from the input. Expects
    /// the current token to be an expression, starting with a literal value or identifier.
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

    /// The `parse_block_statement` method parses a block statement from the input. Expects the
    /// current token to be a `TokenType::LBrace`.
    pub(crate) fn parse_block_statement(&mut self) -> Result<BlockStatement, ()> {
        let token = self.current_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        // We end parsing if we reach the end of the file or a closing brace.
        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }

        Ok(BlockStatement { token, statements })
    }
}
