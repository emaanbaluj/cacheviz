
use crate::parser::token::Token;
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    declared: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub pos: usize,
}

impl ParseError {
    fn new(message: impl Into<String>, pos: usize) -> Self {
        ParseError { message: message.into(), pos }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error at token {}: {}", self.pos, self.message)
    }
}



impl Parser {

    /// Creates a new Parser from the given source vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            declared: Vec::new(),
        }
    }

    /// Root function to convert tokens into AST
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while self.current() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    // Looks at current token and dispatches to the correct parse function
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current() {
            Token::For => self.parse_for_loop(),
            Token::Int => {
                // could be int x = 5;  or  int A[8];
                // peek at the token after the name to decide
                if self.is_array_decl() {
                    self.parse_array_decl()
                } else {
                    self.parse_var_decl()
                }
            }
            Token::Identifier(_) => self.parse_assignment(), // x = 5; or A[i] = i;
            other => Err(ParseError::new(
                format!("unexpected token {:?}", other),
                self.pos,
            )),
        }
    }

    // Peek two tokens ahead: Int <name> [ => array decl, Int <name> = => var decl
    fn is_array_decl(&self) -> bool {
        if self.pos + 2 < self.tokens.len() {
            self.tokens[self.pos + 2] == Token::LBracket
        } else {
            false
        }
    }

    // Helper function for parsing a ForLoop
    // Structure ForLoop:
    // For
    // LParen
    //   Int
    //   Identifier        <- iterator name e.g. "i"
    //   Equals
    //   IntLiteral        <- start value e.g. 0
    //   Semicolon
    //   Identifier        <- same iterator in condition e.g. "i"
    //   Less              <- condition operator e.g. <
    //   IntLiteral        <- end value e.g. 8
    //   Semicolon
    //   Identifier        <- same iterator in increment e.g. "i"
    //   Plus
    //   Plus              <- i++
    // RParen
    // LBrace
    //   ...body statements...
    // RBrace
    fn parse_for_loop(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::For)?;
        self.expect(Token::LParen)?;

        // init: int i = 0;
        self.expect(Token::Int)?;
        let iterator = self.expect_identifier()?;   // "i"
        self.expect(Token::Equals)?;
        let start = self.parse_expression()?;       // 0

        self.expect(Token::Semicolon)?;

        // condition: i < 8;
        self.expect_identifier()?;                  // "i" (skip it, we already have iterator)
        self.expect(Token::Less)?;
        let end = self.parse_expression()?;         // 8

        self.expect(Token::Semicolon)?;

        // increment: i++
        self.expect_identifier()?;                  // "i"
        self.expect(Token::Plus)?;
        self.expect(Token::Plus)?;

        self.expect(Token::RParen)?;

        // iterator is declared by the loop init — register it so body can reference it
        self.declared.push(iterator.clone());

        // body
        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while self.current() != &Token::RBrace {
            if self.current() == &Token::Eof {
                return Err(ParseError::new("unexpected EOF inside for loop body", self.pos));
            }

            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        // check all arrays accessed in the body were declared before the loop
        for stmt in &body {
            if let Statement::Assignment(a) = stmt {
                if let Expression::ArrayAccess(arr) = &a.target {
                    if !self.is_declared(&arr.name) {
                        return Err(ParseError::new(
                            format!("array '{}' used in for loop body was not declared", arr.name),
                            self.pos,
                        ));
                    }
                }
            }
        }

        Ok(Statement::ForLoop(ForLoop { iterator, start, end, body }))
    }


    // Helper function for parsing ArrayDecl
    // Structure ArrayDecl:
    //   Int
    //   Identifier        <- array name e.g. "A"
    //   LBracket
    //   IntLiteral        <- size e.g. 8
    //   RBracket
    //   (LBracket IntLiteral RBracket)  <- optional second dimension e.g. [4]
    //   Semicolon
    fn parse_array_decl(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::Int)?;
        let name = self.expect_identifier()?;   // "A"

        // first dimension: [8]
        self.expect(Token::LBracket)?;
        let first = self.parse_expression()?;   // 8
        self.expect(Token::RBracket)?;

        let mut dimensions = vec![first];

        // optional second dimension: [4]
        if self.current() == &Token::LBracket {
            self.advance();
            dimensions.push(self.parse_expression()?);
            self.expect(Token::RBracket)?;
        }

        self.expect(Token::Semicolon)?;

        self.declared.push(name.clone());
        Ok(Statement::ArrayDecl(ArrayDecl { name, dimensions }))
    }

    // Helper function for parsing VarDecl
    // Structure VarDecl:
    //   Int
    //   Identifier        <- variable name e.g. "x"
    //   Equals
    //   IntLiteral        <- initial value e.g. 5
    //   Semicolon
    fn parse_var_decl(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::Int)?;
        let name = self.expect_identifier()?;
        self.expect(Token::Equals)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        self.declared.push(name.clone());
        Ok(Statement::VarDecl(VarDecl { name, value }))
    }


    // Helper function for parsing Assignment
    // Structure Assignment:
    //   Identifier        <- target variable e.g. "x" or array access "A[i]"
    //   (LBracket Identifier/IntLiteral RBracket)  <- optional index if array
    //   Equals
    //   <expression>      <- value being assigned e.g. 5, i, A[i], i+1
    //   Semicolon
    fn parse_assignment(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;

        // check if target is an array access e.g. A[i] or B[i][j]
        let target = if self.current() == &Token::LBracket {
            let mut indices = Vec::new();
            while self.current() == &Token::LBracket {
                self.advance();
                indices.push(self.parse_expression()?);
                self.expect(Token::RBracket)?;
            }
            Expression::ArrayAccess(ArrayAccess { name, indices })
        } else {
            Expression::Identifier(name)
        };

        self.expect(Token::Equals)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(Statement::Assignment(Assignment { target, value }))
    }

    // Helper function for parsing an Expression
    // An expression is one of:
    //   IntLiteral        <- e.g. 5
    //   Identifier        <- e.g. i, sum
    //   Identifier LBracket <expr> RBracket       <- array access e.g. A[i]
    //   Identifier LBracket <expr> RBracket LBracket <expr> RBracket  <- 2D e.g. B[i][j]
    //   <expr> (Plus|Minus|Star|Slash) <expr>     <- binary op e.g. i + 1
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        // parse the left-hand side first
        let mut left = match self.current().clone() {
            Token::IntLiteral(n) => {
                self.advance();
                Expression::Literal(n)
            }
            Token::Identifier(name) => {
                self.advance();
                // check if followed by [ for array access
                if self.current() == &Token::LBracket {
                    let mut indices = Vec::new();
                    while self.current() == &Token::LBracket {
                        self.advance();
                        indices.push(self.parse_expression()?);
                        self.expect(Token::RBracket)?;
                    }
                    Expression::ArrayAccess(ArrayAccess { name, indices })
                } else {
                    Expression::Identifier(name)
                }
            }
            other => return Err(ParseError::new(
                format!("expected expression but got {:?}", other),
                self.pos,
            )),
        };

        // check if followed by a binary operator
        left = match self.current() {
            Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                let op = format!("{:?}", self.current().clone());
                self.advance();
                let right = self.parse_expression()?;
                Expression::Binary(Box::new(BinaryExpr { left, op, right }))
            }
            _ => left,
        };

        Ok(left)
    }

    // Helper to consume an Identifier token and return its name
    // Returns ParseError if the current token is not an Identifier
    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.current().clone() {
            Token::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            other => Err(ParseError::new(
                format!("expected identifier but got {:?}", other),
                self.pos,
            )),
        }
    }

    // Returns true if name was declared (via VarDecl or ArrayDecl) before this point in parsing
    fn is_declared(&self, name: &str) -> bool {
        self.declared.iter().any(|n| n == name)
    }

    // Checks to see if a specific variable has been assigned anywhere in the program
    fn check_assignment(&self, program: &Program, name: &str) -> bool {
        program.statements.iter().any(|stmt| Self::stmt_has_assignment(stmt, name))
    }

    fn stmt_has_assignment(stmt: &Statement, name: &str) -> bool {
        match stmt {
            Statement::Assignment(a) => match &a.target {
                Expression::Identifier(n) => n == name,
                Expression::ArrayAccess(arr) => arr.name == name,
                _ => false,
            },
            Statement::ForLoop(f) => f.body.iter().any(|s| Self::stmt_has_assignment(s, name)),
            _ => false,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(
                format!("expected {:?} but got {:?}", expected, self.current()),
                self.pos,
            ))
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::lexer::Lexer;

    #[test]
    fn test_var_decl() {
        let tokens = Lexer::new("int x = 5;").tokenize();
        let ast = Parser::new(tokens).parse().unwrap();
        assert_eq!(ast, Program {
            statements: vec![
                Statement::VarDecl(VarDecl {
                    name: "x".to_string(),
                    value: Expression::Literal(5),
                }),
            ],
        });
    }

    #[test]
    fn test_array_decl() {
        let tokens = Lexer::new("int A[8];").tokenize();
        let ast = Parser::new(tokens).parse().unwrap();
        assert_eq!(ast, Program {
            statements: vec![
                Statement::ArrayDecl(ArrayDecl {
                    name: "A".to_string(),
                    dimensions: vec![Expression::Literal(8)],
                }),
            ],
        });
    }

    #[test]
    fn test_assignment() {
        let tokens = Lexer::new("x = 5;").tokenize();
        let ast = Parser::new(tokens).parse().unwrap();
        assert_eq!(ast, Program {
            statements: vec![
                Statement::Assignment(Assignment {
                    target: Expression::Identifier("x".to_string()),
                    value: Expression::Literal(5),
                }),
            ],
        });
    }

    #[test]
    fn test_for_loop_parser() {
        let tokens = Lexer::new("for (int i = 0; i < 10; i++) {}").tokenize();
        let ast = Parser::new(tokens).parse().unwrap();
        assert_eq!(ast, Program {
            statements: vec![
                Statement::ForLoop(ForLoop {
                    iterator: "i".to_string(),
                    start: Expression::Literal(0),
                    end: Expression::Literal(10),
                    body: vec![],
                }),
            ],
        });
    }

    #[test]
    fn test_for_loop_parser_with_assignment() {
        let tokens = Lexer::new("int A[10]; for (int i = 0; i < 10; i++) { A[i] = i; }").tokenize();
        let ast = Parser::new(tokens).parse().unwrap();
        assert_eq!(ast, Program {
            statements: vec![
                Statement::ArrayDecl(ArrayDecl {
                    name: "A".to_string(),
                    dimensions: vec![Expression::Literal(10)],
                }),
                Statement::ForLoop(ForLoop {
                    iterator: "i".to_string(),
                    start: Expression::Literal(0),
                    end: Expression::Literal(10),
                    body: vec![
                        Statement::Assignment(Assignment {
                            target: Expression::ArrayAccess(ArrayAccess {
                                name: "A".to_string(),
                                indices: vec![Expression::Identifier("i".to_string())],
                            }),
                            value: Expression::Identifier("i".to_string()),
                        }),
                    ],
                }),
            ],
        });
    }

    #[test]
    fn test_for_loop_parser_with_no_declaration() {
        // A is never declared before the loop — parser should reject this
        let tokens = Lexer::new("for (int i = 0; i < 10; i++) { A[i] = i; }").tokenize();
        let result = Parser::new(tokens).parse();
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("'A'"));
    }




    #[test]
    fn test_parse_error_unexpected_token() {
        let tokens = Lexer::new("} x = 5;").tokenize();
        let result = Parser::new(tokens).parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.pos, 0);
    }

    #[test]
    fn test_parse_error_missing_semicolon() {
        let tokens = Lexer::new("int x = 5").tokenize();
        let result = Parser::new(tokens).parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Semicolon"));
    }



}
