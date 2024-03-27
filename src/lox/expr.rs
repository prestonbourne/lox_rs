use super::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    This(SourceLocation),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Call(Box<Expr>, SourceLocation, Vec<Expr>),
    Get(Box<Expr>, Symbol),
    Grouping(Box<Expr>),
    Variable(Symbol),
    Assign(Symbol, Box<Expr>),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Set(Box<Expr>, Symbol, Box<Expr>),
    Super(SourceLocation, Symbol),
    List(Vec<Expr>),
    Subscript {
        value: Box<Expr>,
        slice: Box<Expr>,
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
pub enum LogicalOp {
    Or,
    And,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Symbol {
    pub name: String,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct LambdaDecl {
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: Symbol,
    pub superclass: Option<Symbol>,
    pub methods: Vec<FunDecl>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    FunDecl(FunDecl),
    ClassDecl(ClassDecl),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    VarDecl(Symbol, Option<Expr>),
    Block(Vec<Stmt>),
    Return(SourceLocation, Option<Expr>),
    While(Expr, Box<Stmt>),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOpType {
    Minus,
    Bang,
}

#[derive(Debug, Copy, Clone)]
pub struct UnaryOp {
    pub ty: UnaryOpType,
    pub line: usize,
    pub col: i64,
}

impl From<UnaryOp> for String {
    fn from(op: UnaryOp) -> Self {
        match op.ty {
            UnaryOpType::Minus => "-".to_string(),
            UnaryOpType::Bang => "!".to_string(),
        }
    }
}

impl From<Token> for UnaryOp {
    fn from(token: Token) -> Self {
        match token.ty {
            TokenType::Minus => UnaryOp {
                ty: UnaryOpType::Minus,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Bang => UnaryOp {
                ty: UnaryOpType::Bang,
                line: token.line,
                col: token.col as i64,
            },
            _ => panic!("Invalid token type for unary operator: {:?}", token.ty),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOpType {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOp {
    pub ty: BinaryOpType,
    pub line: usize,
    pub col: i64,
}

impl From<BinaryOp> for String {
    fn from(op: BinaryOp) -> Self {
        match op.ty {
            BinaryOpType::EqualEqual => "==".to_string(),
            BinaryOpType::NotEqual => "!=".to_string(),
            BinaryOpType::Less => "<".to_string(),
            BinaryOpType::LessEqual => "<=".to_string(),
            BinaryOpType::Greater => ">".to_string(),
            BinaryOpType::GreaterEqual => ">=".to_string(),
            BinaryOpType::Plus => "+".to_string(),
            BinaryOpType::Minus => "-".to_string(),
            BinaryOpType::Star => "*".to_string(),
            BinaryOpType::Slash => "/".to_string(),
        }
    }
}

impl From<Token> for BinaryOp {
    fn from(token: Token) -> Self {
        match token.ty {
            TokenType::EqualEqual => BinaryOp {
                ty: BinaryOpType::EqualEqual,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::BangEqual => BinaryOp {
                ty: BinaryOpType::NotEqual,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Less => BinaryOp {
                ty: BinaryOpType::Less,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::LessEqual => BinaryOp {
                ty: BinaryOpType::LessEqual,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Greater => BinaryOp {
                ty: BinaryOpType::Greater,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::GreaterEqual => BinaryOp {
                ty: BinaryOpType::GreaterEqual,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Plus => BinaryOp {
                ty: BinaryOpType::Plus,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Minus => BinaryOp {
                ty: BinaryOpType::Minus,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Star => BinaryOp {
                ty: BinaryOpType::Star,
                line: token.line,
                col: token.col as i64,
            },
            TokenType::Slash => BinaryOp {
                ty: BinaryOpType::Slash,
                line: token.line,
                col: token.col as i64,
            },
            _ => panic!("Invalid token type for binary operator: {:?}", token.ty),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
