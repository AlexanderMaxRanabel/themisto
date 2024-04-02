use colored::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

pub fn linear_tokenizer(line: String) -> Vec<String> {
    let str_tokens: Vec<&str> = line.split_whitespace().collect();
    let tokens = str_tokens.iter().map(|&s| s.to_owned()).collect();
    return tokens;
}

pub fn double_tokenizer(tokens: Vec<String>, starter: &str, ender: &str) -> Vec<String> {
    let raw_tokens: Vec<_> = tokens
        .iter()
        .skip_while(|c| c != &&starter)
        .skip(1)
        .take_while(|c| c != &&ender)
        .collect();

    let extracted_tokens: Vec<String> = raw_tokens.iter().map(|&s| s.to_string()).collect();

    return extracted_tokens;
}

pub fn parser(file: String, mut stack: Vec<String>) -> anyhow::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let tokens = linear_tokenizer(line.clone());
        let main_keyword = tokens.get(0).cloned().unwrap_or_else(|| {
            std::process::exit(1);
        });

        match main_keyword.as_str() {
            "alloc_to" => {
                let non_usize_range = tokens.get(1).cloned().unwrap_or_else(|| {
                    println!("{}: No range for allocation of memory provided", "Error".red());
                    std::process::exit(1);
                });

                let range: usize = non_usize_range.trim().parse().expect("Please provide a valid range");
                stack.resize(stack.len() + range, "".to_string());
            }

            "mov" => {
                let basic_info: Vec<String> = double_tokenizer(tokens, "[", "]");
                println!("{:?}", basic_info);
                /*let mut var_type;
                if let Some(index) = tokens.iter().position(|&x| x == "") {
                    if index + 0 < tokens.len() {
                        let next_fruit = tokens[index + 0];
                    } else {
                        println!("There's no fruit after blueberry");
                    }*/
            }

            _ => {
                println!("{}: Unknown keyword: {}", "Error".red(), main_keyword);
                process::exit(1);
            }
        }
    }

    Ok(())
}
