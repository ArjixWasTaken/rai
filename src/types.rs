#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    Pound,
    Comma,
    Colon,
    SemiColon,
    MemberAccess,

    Int,
    Word,
    Float,
    String,
    Comment,

    Operator,
    ComparisonOperator,
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
    pub typ: TokenType,
    pub val: String,
    pub span: Span,
}
