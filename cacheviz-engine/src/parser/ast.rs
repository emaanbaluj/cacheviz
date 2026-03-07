/// Root of the AST — holds all top-level statements in the program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A top-level statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDecl(VarDecl),
    ArrayDecl(ArrayDecl),
    Assignment(Assignment),
    ForLoop(ForLoop),
}

/// A value-producing expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(i64),
    Identifier(String),
    ArrayAccess(ArrayAccess),
    Binary(Box<BinaryExpr>),
}

/// int x = 5;
#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub value: Expression,
}

/// int A[8]; or int B[4][4];
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayDecl {
    pub name: String,
    pub dimensions: Vec<Expression>,
}

/// x = 5; or A[i] = i;
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub target: Expression,
    pub value: Expression,
}

/// for (int i = 0; i < 8; i++) { ... }
#[derive(Debug, Clone, PartialEq)]
pub struct ForLoop {
    pub iterator: String,
    pub start: Expression,
    pub end: Expression,
    pub body: Vec<Statement>,
}

/// A[i] or B[i][j]
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAccess {
    pub name: String,
    pub indices: Vec<Expression>,
}

/// i + 1, i * 2, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Expression,
    pub op: String,
    pub right: Expression,
}
