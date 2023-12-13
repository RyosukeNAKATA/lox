#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    This(SourceLocation),
    Unary(UnaryOperand, Box<Expr>),
    Binary(Box<Expr>, BinaryOperand, Box<Expr>),
    Call(Box<Expr>, SourceLocation, Vec<Expr>),
    Get(Box<Expr>, Symbol),
    Grouping(Box<Expr>),
    Variable(Symbol),
    Assign(Symbol, Box<Expr>),
    Logical(Box<Expr>, LogicalOperand, Box<Expr>),
    Set(Box<Expr>, Symbol, Box<Expr>),
    Super(SourceLocation, Symbol),
    List(Vec<Expr>),
    Subscript {
        value: Box<Expr>,
        slice: slice<Box>,
        source_location: SourceLocation,
    },
    SetItem {
        lhs: Box<Expr>,
        slice: Box<Expr>,
        rhs: Box<Expr>,
        source_location: SourceLocation,
    },
    Lambda(LambdaDecl),
}

#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
pub enum LogicalOperand {
    Or,
    And,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Symbol {
    pub name: String,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOperand {
    pub ty: BinaryOperandType,
    pub line: usize,
    pub col: i64,
}
#[derive(Debug, Copy, Clone)]
pub enum BinaryOperandType {
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

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}
