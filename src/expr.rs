#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, BinaryOprator, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOprator {
    pub ty: BinaryOpratoType,
    pub line: usize,
    pub col: i64,
}
#[derive(Debug, Copy, Clone)]
pub enum BinaryOpratoType {
    EqualEqual,
    NotEqual,
    less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plust,
    Minus,
    Star,
    Slash,
}
