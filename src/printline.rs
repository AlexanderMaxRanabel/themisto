use crate::math_utils;
use colored::*;
use evalexpr::*;

pub fn println(
    mut tokens: Vec<String>,
    stack: Vec<String>,
    heap: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    tokens.remove(0);
    let full_str = tokens.join(" ");
    if full_str.starts_with("'") && full_str.ends_with("'") {
        println!("{}", full_str);
    } else {
        if let Some(first_token) = tokens.get(0) {
            if let Ok(_number) = first_token.parse::<i64>() {
                let edited_full_str = math_utils::equation_restructure(full_str, stack.clone(), heap.clone());
                match eval(edited_full_str.as_str()) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Failed to evaluate expression: {}", e),
                }
            } else {
                if let Some(second_token) = tokens.get(1) {
                    if let Ok(number) = second_token.parse::<usize>() {
                        match first_token.as_str() {
                            "stack" => {
                                let element_from_stack = stack.get(number).unwrap();
                                println!("{}", element_from_stack);
                            }

                            "heap" => {
                                let element_from_heap = heap.get(number).unwrap();
                                println!("{}", element_from_heap);
                            }

                            _ => {
                                println!(
                                    "{}: Unknown Memory Space: {}",
                                    "Error".red(),
                                    first_token
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
        } else {
            println!("No first token found");
        }
    }
    (stack, heap)
}
