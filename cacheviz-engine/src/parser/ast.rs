
/// Node for For Loop
pub struct ForLoop {
    pub iterator: String,
    pub start: Expression,
    pub end: Expression,
    pub body: Vec<Statement>,  // children live here
}

/// Node for Array Decleration
pub struct ArrayDecl {
    pub name: String,
    pub dimensions: Vec<usize>,
}

/// Node for Variable Decleration
pub struct VarDecl {
    pub name: String,
    pub value: Option<Expression>,
}



