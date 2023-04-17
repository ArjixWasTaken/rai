use std::vec;

use crate::types::{Span, Token, TokenType};

macro_rules! parse_operator {
    ($chars: ident, $idx: ident, $rows: ident, $col: ident) => {{
        let mut tok: Option<Token> = None;

        if $chars.len() > $idx {
            $idx += 1;
            if $chars[$idx] == '=' {
                tok = Some(Token {
                    typ: TokenType::Assignment,
                    val: format!("{}=", $chars[$idx - 1]),
                    span: Span {
                        line: $rows.len(),
                        col: $col,
                        length: 2,
                    },
                });
            } else if $chars[$idx - 1] == $chars[$idx] {
                tok = Some(Token {
                    typ: TokenType::Operator,
                    val: format!("{}{}", $chars[$idx - 1], $chars[$idx]),
                    span: Span {
                        line: $rows.len(),
                        col: $col,
                        length: 2,
                    },
                });
            } else {
                $idx -= 1;
            }
        }

        if tok.is_none() {
            tok = Some(Token {
                typ: TokenType::Operator,
                val: $chars[$idx].into(),
                span: Span {
                    line: $rows.len(),
                    col: $col,
                    length: 2,
                },
            });
        }

        tok
    }};
}

pub(crate) fn lex(text: &str) -> Vec<Token> {
    let chars = text.replace("\t", "    ").chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = vec![];
    let mut idx = 0;

    let mut rows: Vec<usize> = vec![];
    let mut col: usize = 1;

    while idx < chars.len() {
        let char: char = chars[idx];
        let val: String = char.into();
        let mut span = Span {
            line: rows.len() + 1,
            length: 1,
            col,
        };

        let tok = match char {
            '#' => Some(Token {
                typ: TokenType::Pound,
                val,
                span,
            }),
            '(' => Some(Token {
                typ: TokenType::OpenParen,
                val,
                span,
            }),
            ')' => Some(Token {
                typ: TokenType::CloseParen,
                val,
                span,
            }),
            '[' => Some(Token {
                typ: TokenType::OpenBracket,
                val,
                span,
            }),
            ']' => Some(Token {
                typ: TokenType::CloseBracket,
                val,
                span,
            }),
            '{' => Some(Token {
                typ: TokenType::OpenBrace,
                val,
                span,
            }),
            '}' => Some(Token {
                typ: TokenType::CloseBrace,
                val,
                span,
            }),
            ',' => Some(Token {
                typ: TokenType::Comma,
                val,
                span,
            }),
            ';' => Some(Token {
                typ: TokenType::SemiColon,
                val,
                span,
            }),
            ':' => {
                let mut tok: Option<Token> = None;

                if let Some(next) = chars.get(idx + 1) {
                    if next == &'=' || next == &':' {
                        idx += 1;

                        span.length = 2;

                        tok = Some(Token {
                            typ: if next == &':' {
                                TokenType::ConstAssignment
                            } else {
                                TokenType::Assignment
                            },
                            val: format!("{}{}", char, next),
                            span,
                        });
                    }
                }

                if tok.is_none() {
                    span.col = col;

                    tok = Some(Token {
                        typ: TokenType::Colon,
                        val,
                        span,
                    });
                }

                tok
            }
            '-' => parse_operator!(chars, idx, rows, col),
            '*' => parse_operator!(chars, idx, rows, col),
            '+' => parse_operator!(chars, idx, rows, col),
            '%' => parse_operator!(chars, idx, rows, col),
            '"' => {
                let mut string_chars: Vec<char> = vec![];
                idx += 1;

                let mut prev = chars.get(idx).unwrap_or(&'.');

                while idx < chars.len() {
                    if chars[idx] == '"' {
                        if prev != &'\\' {
                            break;
                        }
                    }

                    if prev == &'\\' {
                        match chars[idx] {
                            // Replaces some escaped characters with their actual value.
                            'n' => {
                                string_chars.pop();
                                string_chars.push('\n');
                                col += 1;
                            }
                            't' => {
                                string_chars.pop();
                                string_chars.push('\t');
                                col += 1;
                            }
                            'r' => {
                                string_chars.pop();
                                string_chars.push('\r');
                                col += 1;
                            }
                            '"' => {
                                string_chars.pop();
                                string_chars.push('"');
                                col += 1;
                            }
                            _ => string_chars.push(chars[idx]),
                        }
                    } else {
                        string_chars.push(chars[idx])
                    }

                    prev = &chars[idx];
                    idx += 1;
                }

                if chars[idx] != '"' {
                    panic!("Missing quote from string: Did you forget to add a closing quote to the string?")
                }

                span.col = col;
                span.length = string_chars.len() + 2;

                Some(Token {
                    typ: TokenType::String,
                    val: string_chars.iter().collect(),
                    span,
                })
            }
            '/' => {
                if let Some(next) = chars.get(idx + 1) {
                    if next == &'*' || next == &'/' {
                        let mut comment_chars: Vec<char> = vec![];
                        let is_multiline = next == &'*';

                        idx += 2;
                        if idx >= chars.len() {
                            unreachable!()
                        }
                        let mut prev = chars.get(idx).unwrap_or(&'.');

                        while idx < chars.len()
                            && ((is_multiline && (prev != &'*' && chars[idx] != '/'))
                                || (!is_multiline && chars[idx] != '\n'))
                        {
                            comment_chars.push(chars[idx]);
                            prev = &chars[idx];
                            idx += 1;

                            if prev == &'\n' {
                                rows.push(idx + 1 - rows.iter().sum::<usize>());
                                col = 1;
                            }
                        }

                        if is_multiline {
                            if chars[idx - 1] == '*' && chars[idx] == '/' {
                                comment_chars.pop();
                                idx += 1;
                            } else {
                                panic!("Multiline comment was not closed.")
                            }
                        }
                        idx -= 1;

                        span.length = comment_chars.len() + 2;
                        span.col = col;

                        Some(Token {
                            typ: TokenType::Comment,
                            val: comment_chars.into_iter().collect(),
                            span,
                        })
                    } else {
                        parse_operator!(chars, idx, rows, col)
                    }
                } else {
                    unreachable!()
                }
            }
            '.' => {
                if let Some(next) = chars.get(idx + 1) {
                    if next.is_numeric() {
                        let mut number_chars: Vec<char> = vec!['0', '.'];
                        idx += 1;

                        while idx < chars.len() && chars[idx].is_numeric() {
                            number_chars.push(chars[idx]);
                            idx += 1;
                        }

                        idx -= 1;

                        span.length = number_chars.len();
                        span.col = col;

                        Some(Token {
                            typ: TokenType::Float,
                            val: number_chars.into_iter().collect(),
                            span,
                        })
                    } else if next.is_alphabetic() || next == &'_' {
                        Some(Token {
                            typ: TokenType::MemberAccess,
                            val: char.to_string(),
                            span,
                        })
                    } else {
                        unreachable!("You can't have a dot w/o purpose, that's just sad.");
                    }
                } else {
                    unreachable!("What the fuck is a dot doing at the end of the file?");
                }
            }
            _ => {
                if ['=', '<', '>'].contains(&chars[idx]) {
                    let mut tok = Token {
                        typ: TokenType::ComparisonOperator,
                        val: chars[idx].to_string(),
                        span,
                    };

                    if let Some(next) = chars.get(idx + 1) {
                        if next == &'=' {
                            tok.val = format!("{}=", chars[idx]);
                            idx += 1;
                        }
                    }

                    if tok.val == "=" {
                        tok.typ = TokenType::Assignment;
                    }

                    tok.span.length = tok.val.len();

                    Some(tok)
                } else if chars[idx].is_numeric() {
                    let mut number_chars: Vec<char> = vec![chars[idx]];
                    let mut is_float = false;
                    idx += 1;

                    while idx < chars.len() && (chars[idx].is_numeric() || chars[idx] == '.') {
                        if chars[idx] == '.' && is_float {
                            idx -= 1;
                            is_float = false;

                            number_chars.pop();
                            break;
                        } else if chars[idx] == '.' {
                            is_float = true;
                        }

                        number_chars.push(chars[idx]);
                        idx += 1;
                    }

                    idx -= 1;

                    span.length = number_chars.len();
                    span.col = col;

                    Some(Token {
                        typ: (if is_float {
                            TokenType::Float
                        } else {
                            TokenType::Int
                        }),
                        val: number_chars.into_iter().collect(),
                        span,
                    })
                } else if idx + 1 < chars.len() && chars[idx] == '.' && chars[idx + 1] == '.' {
                    idx += 1;

                    span.col = col;
                    span.length = 2;

                    Some(Token {
                        typ: TokenType::Operator,
                        val: "Range".into(),
                        span,
                    })
                } else if chars[idx].is_whitespace() {
                    if chars[idx] == '\n' {
                        rows.push(idx + 1 - rows.iter().sum::<usize>());
                        col = 1;
                    } else {
                        col += 1;
                    }

                    None
                } else if chars[idx].is_alphabetic() || chars[idx] == '_' {
                    let mut word_chars: Vec<char> = vec![];

                    while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '_') {
                        word_chars.push(chars[idx]);
                        idx += 1;
                    }

                    idx -= 1;
                    let word = word_chars.iter().collect::<String>();

                    span.length = word_chars.len();
                    span.col = col;

                    Some(Token {
                        typ: TokenType::Word,
                        val: word,
                        span,
                    })
                } else {
                    panic!(
                        "Unexpected value found: {}",
                        chars[idx..].into_iter().take(20).collect::<String>()
                    )
                }
            }
        };

        idx += 1;

        if let Some(tok) = tok {
            col += span.length;
            tokens.push(tok)
        }
    }

    rows.push(idx + 1 - rows.iter().sum::<usize>());

    tokens
}
