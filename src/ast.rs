use crate::spans::{Span, Spanned};

#[derive(Debug)]
pub enum Literal {
    Bool,
    Float,
    Int,
    Str,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Rem,

    Lt,
    Lte,
    Gt,
    Gte,

    Eq,
    Neq,
}

#[derive(Debug)]
pub enum OpType {
    IntOp,
    FloatOp,
    StrOp,

    IntCmp,
    FloatCmp,
    AnyCmp,
}

type VarDefinition = (String, Box<Expr>);

#[derive(Debug)]
pub enum Pattern {
    Case(String, String),
    Wildcard(String),
}

#[derive(Debug)]
pub enum Expr {
    BinOp(Spanned<Box<Expr>>, Spanned<Box<Expr>>, OpType, Op, Span),
    Call(Box<Expr>, Box<Expr>, Span),
    Case(Spanned<String>, Box<Expr>),
    FieldAccess(Box<Expr>, String, Span),
    FuncDef(Spanned<(String, Box<Expr>)>),
    If(Spanned<Box<Expr>>, Box<Expr>, Box<Expr>),
    Let(VarDefinition, Box<Expr>),
    LetRec(Vec<VarDefinition>, Box<Expr>),
    Literal(Literal, Spanned<String>),
    Match(Box<Expr>, Vec<(Spanned<Pattern>, Box<Expr>)>, Span),
    NewRef(Box<Expr>, Span),
    Record(Option<Box<Expr>>, Vec<(Spanned<String>, Box<Expr>)>, Span),
    RefGet(Spanned<Box<Expr>>),
    RefSet(Spanned<Box<Expr>>, Box<Expr>),
    Variable(Spanned<String>),
}

#[derive(Debug)]
pub enum TopLevel {
    Expr(Expr),
    LetDef(VarDefinition),
    LetRecDef(Vec<VarDefinition>),
}
