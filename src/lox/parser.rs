use core::panic;
use std::fmt;

use super::expr::{Expr, Literal};
use super::token;
use super::token::{Token, TokenType};

/*
In order of precedence
+------------+------------+------------+
| Name       | Operators  | Associates |
+------------+------------+------------+
| Equality   | == !=      | Left       |
| Comparison | > >= < <=  | Left       |
| Term       | - +        | Left       |
| Factor     | / *        | Left       |
| Unary      | ! -        | Right      |
+------------+------------+------------+

expression     → literal | unary | binary | grouping ;
literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;


 example expression grammar rule

 (12 + 1) * 3 > 4
 will produce the following tokens:
    [
        Token { ty: TokenType::LeftParen, lexeme: "(", literal: None, line: 1 },
        Token { ty: TokenType::Number, lexeme: "12", literal: Some(Literal::Number(12.0)), line: 1 },
        Token { ty: TokenType::Plus, lexeme: "+", literal: None, line: 1 },
        Token { ty: TokenType::Number, lexeme: "1", literal: Some(Literal::Number(1.0)), line: 1 },
        Token { ty: TokenType::RightParen, lexeme: ")", literal: None, line: 1 },
        Token { ty: TokenType::Star, lexeme: "*", literal: None, line: 1 },
        Token { ty: TokenType::Number, lexeme: "3", literal: Some(Literal::Number(3.0)), line: 1 },
        Token { ty: TokenType::Greater, lexeme: ">", literal: None, line: 1 },
        Token { ty: TokenType::Number, lexeme: "4", literal: Some(Literal::Number(4.0)), line: 1 },
        Token { ty: TokenType::Eof, lexeme: "", literal: None, line: 1 },
    ]

    first the parser will point to the first token:
    Token { ty: TokenType::LeftParen, lexeme: "(", literal: None, line: 1 }
    second the parser will invoke the expression method (which just invokes equality for now):

    ```
    fn expression(&mut self) -> expr::Expr {
        self.equality()
    }
    ```

    1. where it get's interesting is when the parser invokes the equality method:
    2. the parser invokes the methods down the table of precedence until it reaches the bottom where a literal is returned
    3. the parser then starts to build the AST from the bottom up and moves to the next token
    4. now that we are on 12, the parser will invoke down the table of precedence until it reaches the bottom where a literal is returned
    5. Now we are on the + token, when it gets consumed via `match`
    6

    The parser will then produce the following AST:
    Binary(
        Binary(
            Grouping(
                Binary(
                    Literal(Number(12.0)),
                    Plus,
                    Literal(Number(1.0))
                )
            ),
            Star,
            Literal(Number(3.0))
        ),
        Greater,
        Literal(Number(4.0))
    )

*/

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator.into(), Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();

            // Call `self.term()` inside the loop and handle the potential error
            let right = self.term()?;

            expr = Expr::Binary(Box::new(expr), operator.into(), Box::new(right));
        }

        Ok(expr) // Return the result as an Ok value
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator.into(), Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator.into(), Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            let expr = Expr::Unary(operator.into(), Box::new(right));
            Ok(expr)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&[TokenType::False]) {
            let expr = Expr::Literal(Literal::Boolean(false));
            Ok(expr)
        } else if self.match_token(&[TokenType::True]) {
            let expr = Expr::Literal(Literal::Boolean(true));
            Ok(expr)
        } else if self.match_token(&[TokenType::Nil]) {
            let expr = Expr::Literal(Literal::Nil);
            Ok(expr)
        } else if self.match_token(&[TokenType::Number, TokenType::String]) {
            let token_literal = self
                .previous()
                .clone()
                .literal
                .expect("TODO: Semantic error message");

            let expr_literal = match token_literal {
                token::Literal::Number(n) => Literal::Number(n),
                token::Literal::Str(s) => Literal::String(s),
                _ => panic!("Unexpected error"),
            };
            Ok(Expr::Literal(expr_literal))
        } else if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            match expr {
                Ok(expr) => return Ok(Expr::Grouping(Box::new(expr))),
                Err(e) => return Err(e),
            }
        } else {
            let err_msg = format!("Unexpected Token type {:?}", self.peek().ty);
            Err(ParserError::new(&self.peek(), &err_msg))
        }
    }

    // Small helper functions (tags: parser_helpers, parser_utils)

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(&token_type) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::new(&self.peek().clone(), message))
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ty == *token_type
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    // End of helper functions
    // Start of error handling functions
    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ty == TokenType::Semicolon {
                return;
            }

            match self.peek().ty {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    token: Token,
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "[line {}, col {}] ParserError: {}",
            self.line, self.column, self.message
        );
    }
}

impl ParserError {
    fn new(token: &Token, message: &str) -> ParserError {
        ParserError {
            token: token.clone(),
            message: message.to_string(),
            line: token.line,
            column: token.col,
        }
    }

    fn report(&self) {
        if self.token.ty == TokenType::Eof {
            eprintln!("{} at end of file", self.to_string());
            panic!();
        }
        eprintln!("{}", self.to_string());
        panic!();
    }
}

pub fn stringify_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, operator, right) => {
            let op_string = String::from(*operator);
            format!(
                "({} {} {})",
                op_string,
                stringify_ast(left),
                stringify_ast(right)
            )
        }
        Expr::Grouping(expr) => format!("(group {})", stringify_ast(expr)),
        Expr::Literal(literal) => match literal {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        },
        Expr::Unary(operator, right) => {
            format!("({} {})", String::from(*operator), stringify_ast(right))
        }
        _ => panic!("Unexpected error"),
    }
}

// impl AstPrinter {
//     pub fn stringify(expr: &Expr) -> String {
//         match expr {
//             Expr::Binary(left, operator, right) => {
//                 let op_string = String::from(*operator);
//                 format!(
//                     "({} {} {})",
//                     op_string,
//                     stringify_ast(left),
//                     stringify_ast(right)
//                 )
//             }
//             Expr::Grouping(expr) => format!("(group {})", stringify_ast(expr)),
//             Expr::Literal(literal) => match literal {
//                 Literal::Number(n) => n.to_string(),
//                 Literal::String(s) => s.to_string(),
//                 Literal::Boolean(b) => b.to_string(),
//                 Literal::Nil => "nil".to_string(),
//             },
//             Expr::Unary(operator, right) => {
//                 format!("({} {})", String::from(*operator), stringify_ast(right))
//             }
//             _ => panic!("Unexpected error"),
//         }
//     }
// }
