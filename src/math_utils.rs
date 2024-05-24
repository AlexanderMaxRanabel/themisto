extern crate regex;

use colored::*;
use regex::Regex;
use crate::them_tokenizer;

pub fn equation_restructure(mut expr: String, stack: Vec<String>, heap: Vec<String>) -> String {
    let stack_re = Regex::new(r"stack\((\d+)\)").unwrap();
    let heap_re = Regex::new(r"heap\((\d+)\)").unwrap();

    let mut stack_values = Vec::new();
    let mut heap_values = Vec::new();

    for cap in stack_re.captures_iter(expr.as_str()) {
        if let Some(m) = cap.get(1) {
            if let Ok(value) = m.as_str().parse::<i64>() {
                stack_values.push(value);
            }
        }
    }

    for cap in heap_re.captures_iter(expr.as_str()) {
        if let Some(m) = cap.get(1) {
            if let Ok(value) = m.as_str().parse::<i64>() {
                heap_values.push(value);
            }
        }
    }

    let expr_tokens: Vec<String> = them_tokenizer::linear_tokenizer(expr.clone());
    let (mut stack_indicator, mut heap_indicator) = (0, 0);

    for token in expr_tokens {
        if token.starts_with("stack(") && token.ends_with(")"){
            stack_indicator += 1;
            let stack_metadata = stack.get(stack_indicator - 1 as usize).cloned().unwrap_or_else(|| {
                println!("{}: No Stack Value found", "Error".red());
                std::process::exit(1)
            });
            
            let stack_parts: Vec<&str> = stack_metadata.split('.').collect();
            let stack_value = stack_parts.get(1).cloned().unwrap();

            expr = expr.replace(token.as_str(), stack_value);

        } else if token.starts_with("heap(") && token.ends_with(")") {
            heap_indicator += 1;

            let heap_metadata = heap.get(heap_indicator - 1 as usize).cloned().unwrap_or_else(|| {
                println!("{}: No Heap Value found", "Error".red());
                std::process::exit(1)
            });

            let heap_parts: Vec<&str> = heap_metadata.split('.').collect();
            let heap_value = heap_parts.get(1).cloned().unwrap();

            expr = expr.replace(token.as_str(), heap_value);

        } else {
            continue;
        } 
    }

    return expr;
}
