#[derive(Clone)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Bool(bool),
    Variable(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Gt,
}

#[derive(Clone)]
pub enum Statement {
    Says(Expr),
    Assign(String, Expr),
    If(Expr, Vec<Statement>, Option<Vec<Statement>>),
    DoWhile(Expr, Vec<Statement>),
    Def(String, Vec<String>, Vec<Statement>),
    Return(Expr),
    ExprStatement(Expr),
}
