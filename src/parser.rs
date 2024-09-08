/*
We'll tokenize the code first and then pass it to the parser.
Parser will then just follow the Wab spec and parse it in a left to right manner.
*/
#![allow(unused)]
#![allow(unused_variables)]
use std::{iter::Peekable, vec::IntoIter};

use crate::{
    model::{
        expressions::{Expr, RelOp, Variable},
        statements::{Assignment, If, Print, Return, Stmt, While},
        Program, RelOperator,
    },
    tokenizer::{Token, TokenType},
};

struct Parser {
    tokens_iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn parse(&mut self) -> Program {
        let stmts: Vec<Stmt> = self.parse_stmts();
        Program { stmts }
    }

    fn expect(&mut self, expected_token_type: TokenType) -> Token {
        if let Some(token) = self.tokens_iter.next() {
            if token.token_type == expected_token_type {
                token
            } else {
                panic!(
                    "Expected {:?}, found {:?}",
                    expected_token_type, token.token_type
                );
            }
        } else {
            unreachable!("Should not reach EOF here")
        }
    }

    fn parse_print(&mut self) -> Print {
        self.expect(TokenType::Print);
        let value = self.parse_expr();
        self.expect(TokenType::Semi);

        Print::new(value)
    }

    fn parse_relop(&mut self) -> RelOp {
        let left = self.parse_expr();
        let op = match self.tokens_iter.next() {
            Some(token) => match token.token_type {
                TokenType::Lt => RelOperator::LessThan,
                TokenType::Eq => RelOperator::Equal,
                _ => panic!(
                    "Expected a relational operator, found {:?}",
                    token.token_type
                ),
            },
            None => panic!("Unexpected end of input"),
        };
        let right = self.parse_expr();

        RelOp::new(op, left, right)
    }

    fn parse_assignment_with_decl(&mut self) -> Assignment {
        self.expect(TokenType::Var);
        let var_name = self.expect(TokenType::Name).token_str;
        self.expect(TokenType::Assign);
        let value = self.parse_expr();

        Assignment::new_var(var_name.as_str().into(), value)
    }

    fn parse_assignment_without_decl(&mut self) -> Assignment {
        self.expect(TokenType::Var);
        let var_name = self.expect(TokenType::Name).token_str;
        self.expect(TokenType::Assign);
        let value = self.parse_expr();

        Assignment::assign_var(var_name.as_str().into(), value)
    }

    fn parse_while(&mut self) -> While {
        self.expect(TokenType::While);
        let condition = self.parse_relop();
        self.expect(TokenType::Lbrace);
        let body = self.parse_stmts();
        self.expect(TokenType::Rbrace);

        While::new(condition, body)
    }

    fn parse_if(&mut self) -> If {
        self.expect(TokenType::If);
        let condition = self.parse_relop();
        self.expect(TokenType::Lbrace);
        let consequence = self.parse_stmts();
        self.expect(TokenType::Rbrace);
        self.expect(TokenType::Else);
        self.expect(TokenType::Lbrace);
        let alternative = self.parse_stmts();
        self.expect(TokenType::Rbrace);

        If::new(condition, consequence, alternative)
    }

    fn parse_return(&mut self) -> Return {
        self.expect(TokenType::Return);
        let value = self.parse_expr();

        Return::new(value)
    }

    fn parse_stmts(&mut self) -> Vec<Stmt> {
        todo!()
    }

    /*
    This should parse the following types of statements:

    print 1;
    var x = 1;
    x = 1;
    if 1 == 1 { } else { }
    while 1 == 1 { }
    func f(x) { }
    return 1;
    */
    fn parse_stmt(&mut self) -> Stmt {
        todo!()
    }

    fn parse_expr(&mut self) -> Expr {
        todo!()
    }
}
