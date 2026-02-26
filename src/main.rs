use std::fs::{self};

use bumpalo::Bump;

use crate::{error::Diagnostics, lexer::Lexer, parser::Parser, token::kind::TokenKind};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod pretty;
pub mod span;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = "io/test.se";

    let src = fs::read_to_string(file)?;
    let mut diag = Diagnostics::new(file.to_string(), &src);

    let tokens = Lexer::new(&src, &mut diag).lex();

    // For debugging
    fs::write("io/tokens.txt", {
        tokens
            .clone()
            .into_iter()
            .map(|tok| {
                format!(
                    "{}{:?}<{:?}> = `{}`",
                    if tok.kind.is_unknown() { "!! " } else { "" },
                    tok.kind,
                    tok.span,
                    tok.debug_src(&src),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    let arena = Bump::new();
    let ast = Parser::new(&src, &tokens, &mut diag, &arena).parse();

    // For debugging
    pretty::write_file("io/ast.txt".to_string(), &ast)?;
    pretty::print(&ast)?;

    println!();
    diag.print();

    Ok(())
}
