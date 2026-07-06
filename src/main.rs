use std::fs::{self};

use bumpalo::Bump;

use crate::{
    core::arena_pool::ArenaPool, error::Diagnostics, lexer::Lexer, parser::Parser,
    timing::Stopwatch, token::kind::TokenKind,
};

pub mod ast;
pub mod core;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod pretty;
pub mod timing;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_debug = cfg!(debug_assertions);

    let file = if is_debug { "io/test.se" } else { "io/huge_errorless.se" };

    let mut watch = Stopwatch::start();

    let src = fs::read_to_string(file)?;
    let token_arena = Bump::new();
    let ast_arena = Bump::new();
    let mut diag = Diagnostics::new(file.to_string(), &src);

    watch.split("File Reading");

    let tokens = {
        let tokens = Lexer::new(&src, &mut diag).lex();

        // For debugging
        if is_debug {
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
        }

        tokens
    };

    watch.split("Lexing");

    let ast = {
        let ast = Parser::new(&src, &tokens, &mut diag, &ast_arena).parse();

        if is_debug {
            pretty::write_file("io/ast.txt".to_string(), &ast)?;
            // pretty::print(&ast)?;

            println!();
            diag.print();
        }
        ast
    };

    watch.split("Parsing");

    drop(tokens);
    drop(token_arena);
    watch.split("Token Stream Deallocation");

    drop(ast); // temporarily
    drop(ast_arena);
    watch.split("AST Deallocation");

    let total_time = watch.dump();
    println!(
        "{} LOC/s",
        (src.chars().filter(|&c| c == '\n').count() as f64 / total_time.as_secs_f64())
            as u64
    );

    Ok(())
}
