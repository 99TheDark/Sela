use std::{env, fs};

use bumpalo::Bump;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    error::Diagnostics, lexer::Lexer, parser::Parser, timing::Stopwatch,
    token::kind::TokenKind,
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
    let file = if cfg!(debug_assertions) {
        "io/current.se"
    } else {
        "io/tests/medium_errorless.se"
    };
    let src = fs::read_to_string(file)?;

    if !cfg!(debug_assertions) && args[1..].contains(&"iter".to_string()) {
        const COLD_RUNS: u64 = 3;
        const WARN_RUNS: u64 = 100;

        for i in 0..COLD_RUNS {
            compile(file.to_string(), &src)?;
            println!("Cold run {} / {}\n", i + 1, COLD_RUNS);
        }

        let mut total_loc_per_s = 0;
        let mut total_mb_per_s = 0;
        for i in 0..WARN_RUNS {
            let (loc_per_s, mb_per_s) = compile(file.to_string(), &src)?;
            total_loc_per_s += loc_per_s;
            total_mb_per_s += mb_per_s;
            println!("Warm run {} / {}\n", i + 1, WARN_RUNS);
        }
        println!("--- TOTAL ---");
        println!(
            "{} LOC/s; {} MB/s",
            total_loc_per_s / WARN_RUNS,
            total_mb_per_s / WARN_RUNS
        );
    } else {
        use std::io;

        #[derive(Debug)]
        pub enum CompileError {
            Io(io::Error),
            Pretty(pretty::Error),
        }

        impl From<io::Error> for CompileError {
            fn from(err: io::Error) -> Self {
                CompileError::Io(err)
            }
        }

        impl From<pretty::Error> for CompileError {
            fn from(err: pretty::Error) -> Self {
                CompileError::Pretty(err)
            }
        }

        let srcs = [
            "io/tests/medium_errorless.se",
            "io/tests/huge_errorless.se",
            "io/tests/huge_errorless2.se",
        ];

        let diags: Vec<Result<(&str, usize, usize, Stopwatch), CompileError>> = srcs
            .par_iter()
            .map(|path| -> Result<(&str, usize, usize, Stopwatch), CompileError> {
                let src = fs::read_to_string(path)?;
                let mut diag = Diagnostics::new(path.to_string(), &src);
                let mut arena = Bump::new();
                arena.reset();

                let mut watch = Stopwatch::start();

                let tokens = Lexer::new(&src, &mut diag).lex();
                watch.split("Lexing");
                let ast = Parser::new(&src, &tokens, &mut diag, &arena).parse();
                watch.split("Parsing");

                // pretty::write_file(format!("io/{}", path.replace('/', "-")), &ast)?;

                diag.print();
                watch.split("Error Reporting");
                Ok((path, src.chars().filter(|c| *c == '\n').count(), src.len(), watch))
            })
            .collect();

        for diag in diags {
            match diag {
                Ok((path, loc, size, watch)) => {
                    println!(
                        "Timing for {}: ({} MLOC, {} MB)",
                        path,
                        loc as f64 / 1_000_000f64,
                        size as f64 / 1_000_000f64
                    );
                    watch.dump();
                    println!("");
                }
                Err(err) => match err {
                    CompileError::Io(error) => return Err(Box::new(error)),
                    CompileError::Pretty(error) => return Err(Box::new(error)),
                },
            }
        }

        // compile(file.to_string(), &src)?;
    }

    Ok(())
}

fn compile(file: String, src: &str) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    let mut ast_arena = Bump::new();
    let mut diag = Diagnostics::new(file, &src);

    let mut watch = Stopwatch::start();
    {
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
                pretty::print(&ast)?;

                println!();
                diag.print();
            }
            ast
        };

        watch.split("Parsing");

        drop(tokens);
        watch.split("Token Stream Deallocation");
    }

    ast_arena.reset();
    watch.split("AST Deallocation");

    let line_count = src.chars().filter(|&c| c == '\n').count();
    let byte_count = src.len();

    let total = watch.dump();
    let loc_per_s = (line_count as f64 / total.as_secs_f64()) as u64;
    let mb_per_s = (byte_count as f64 / 1_000_000f64 / total.as_secs_f64()) as u64;
    println!(
        "{} LOC/s; {} MB/s ({} LOC / {} MB total)",
        loc_per_s, mb_per_s, line_count, byte_count
    );

    Ok((loc_per_s, mb_per_s))
}
