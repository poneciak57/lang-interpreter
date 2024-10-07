use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use lang_interpreter::{context::CtxTree, evaluator::Eval, lexer::Lexer, parser::Parser as MyParser};
use miette::{Context, IntoDiagnostic};

#[derive(Debug, Subcommand)]
enum Commands {
    /// Tokenizes the input file
    Tokenize {
        /// Path to a file that u want to tokenize
        filename: PathBuf
    },
    /// Parses the AST from an input file
    Parse {
        /// Path to a file that u want to parse
        filename: PathBuf
    },
    /// Evaluates single expression
    Eval {
        /// Path to a file that u want to evaluate
        filename: PathBuf
    },
    /// Runs the code
    Run {
        /// Path to a file that u want to run
        filename: PathBuf
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            let lexer = Lexer::new(&file_contents);
            for token in lexer {
                let token = token?;
                println!("{token}");
            }
            println!("EOF");
        },
        Commands::Parse { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            let parser = MyParser::new(&file_contents);
            let parsed_str = parser.parse()?
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            println!("{}", parsed_str)
        },
        Commands::Eval { filename} => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;
            let mut parser = MyParser::new(&file_contents);
            let expr_tree = parser.parse_expression_within(0)?;
            let ctx = CtxTree::new();
            let val = expr_tree.eval(&ctx)?;
            println!("evaluation completed");
            println!("result: {val}");
        },
        Commands::Run { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;
            let parser = MyParser::new(&file_contents);
            let expr_list = parser.parse()?;
            let ctx = CtxTree::new();
            for expr in expr_list {
                expr.eval(&ctx)?;
            }
        },
    }

    Ok(())
}
