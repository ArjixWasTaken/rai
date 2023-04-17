#[derive(Debug, Copy, Clone)]
pub enum TokeType {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    Colon,
    SemiColon,

    Int,
    Word,
    Float,
    String,
    Comment,

    Operator,
    Assignment,
    ConstAssignment,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokeType,
    pub val: String,
    pub span: Span,
}
