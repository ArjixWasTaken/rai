#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

#[allow(unused_imports)]
mod lexer;
mod types;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .replace("\r", "");

    let tokens = lexer::lex(&src);

    println!("{:#?}", tokens);
}
