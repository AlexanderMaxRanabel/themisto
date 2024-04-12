use colored::*;
use std::{env, process};

mod mem_manage;
mod mov;
mod parser;
mod them_runtime;
mod them_tokenizer;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let stack: Vec<String> = Vec::new();
        let heap: Vec<String> = Vec::new();
        let file = args.get(1).cloned().unwrap_or_else(|| {
            println!("No file provided");
            std::process::exit(1);
        });

        let _ = parser::parser(file, stack, heap)?;
    } else {
        println!("{}: No File Name Provided", "Error".red());
        process::exit(1);
    }

    Ok(())
}
