
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {

    /// Creates a new Parser from the given source vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    /// Root function to convert tokens into AST
    fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.current() != Token::Eof {
            statements.push(self.parse_statement());
        }
        Program { statements }
    }

    // Looks at current token and dispatches to the correct parse function
    fn parse_statement(&mut self) -> Statement {
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
            _ => panic!("unexpected token"),
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
    fn parse_for_loop() {

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
    fn parse_array_decl() {

    }

    // Helper function for parsing VarDecl
    // Structure VarDecl:
    //   Int
    //   Identifier        <- variable name e.g. "x"
    //   Equals
    //   IntLiteral        <- initial value e.g. 5
    //   Semicolon
    fn parse_var_decl() {

    }

    // Helper function for parsing Assignment
    // Structure Assignment:
    //   Identifier        <- target variable e.g. "x" or array access "A[i]"
    //   (LBracket Identifier/IntLiteral RBracket)  <- optional index if array
    //   Equals
    //   <expression>      <- value being assigned e.g. 5, i, A[i], i+1
    //   Semicolon
    fn parse_assignment() {

    }

    // Helper function for parsing an Expression
    // An expression is one of:
    //   IntLiteral        <- e.g. 5
    //   Identifier        <- e.g. i, sum
    //   Identifier LBracket <expr> RBracket       <- array access e.g. A[i]
    //   Identifier LBracket <expr> RBracket LBracket <expr> RBracket  <- 2D e.g. B[i][j]
    //   <expr> (Plus|Minus|Star|Slash) <expr>     <- binary op e.g. i + 1
    fn parse_expression() {

    }

    // Helper to consume an Identifier token and return its name
    // Panics if the current token is not an Identifier
    fn expect_identifier() {

    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) {
        if self.current() == &expected {
            self.advance();
        } else {
            panic!("expected {:?} but got {:?}", expected, self.current());
        }
    }
}
