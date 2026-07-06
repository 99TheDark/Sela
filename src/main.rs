use std::{env, fs, mem};

use bumpalo::Bump;

use crate::{
    error::Diagnostics,
    lexer::Lexer,
    parser::Parser,
    timing::Stopwatch,
    token::{Token, kind::TokenKind},
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
    let args: Vec<String> = env::args().collect();
    let file = if cfg!(debug_assertions) { "io/test.se" } else { "io/huge_errorless.se" };

    if !cfg!(debug_assertions) && args[1..].contains(&"iter".to_string()) {
        const K: u64 = 25;
        let mut total_loc_per_s = 0;
        for i in 0..K {
            let loc_per_s = compile(file)?;
            total_loc_per_s += loc_per_s;
            println!("{} LOC/s", loc_per_s);
            println!("{} / {}\n", i + 1, K);
        }
        println!("--- TOTAL ---");
        println!("{} LOC/s", total_loc_per_s / K);
    } else {
        let loc_per_s = compile(file)?;
        println!("{} LOC/s", loc_per_s);
    }

    Ok(())
}

fn compile(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut watch = Stopwatch::start();

    let src = fs::read_to_string(file)?;
    let token_arena = Bump::new();
    let ast_arena = Bump::new();
    let mut diag = Diagnostics::new(file.to_string(), &src);

    watch.split("File Reading");

    let tokens = {
        let tokens = Lexer::new(&src, &mut diag).lex();

        // For debugging
        if cfg!(debug_assertions) {
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

        if cfg!(debug_assertions) {
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

    let line_count = src.chars().filter(|&c| c == '\n').count();
    let loc_per_s = (line_count as f64 / watch.dump().as_secs_f64()) as u64;
    Ok(loc_per_s)
}
