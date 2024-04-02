use colored::*;
use std::{env, process};

mod parser;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut stack: Vec<String> = Vec::new();
        let file = args.get(1).cloned().unwrap_or_else(|| {
            println!("No file provided");
            std::process::exit(1);
        });

        let _ = parser::parser(file, stack)?;
    } else {
        println!("{}: No File Name Provided", "Error".red());
        process::exit(1);
    }

    Ok(())
}
