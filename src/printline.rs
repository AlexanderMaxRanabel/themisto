use crate::them_tokenizer;
use colored::*;
use evalexpr::*;

pub fn finder(mut expr: String, stack: Vec<String>, heap: Vec<String>) -> String {
    //TODO: Optimize using regex
    let expr_tokens: Vec<String> = them_tokenizer::linear_tokenizer(expr.clone());
    if expr.contains("stack") {
        let cell_token_v: Vec<String> =
            them_tokenizer::double_tokenizer(expr_tokens.clone(), "stack(", ")");
        let cell_token_u = cell_token_v.get(1).cloned().unwrap_or_else(|| {
            println!("{}: No token provided", "Error".red());
            std::process::exit(1);
        });

        let cell_token_c: usize = cell_token_u.clone().parse().expect("Failed to parse");
        let cell_token_metadata: &String = &stack[cell_token_c];
        let parts: Vec<&str> = cell_token_metadata.split('.').collect();
        let value = parts.get(0).cloned().unwrap_or_else(|| {
            println!("{}: No token provided", "Error".red());
            std::process::exit(1);
        });

        let stack_place: String = format!("stack({})", cell_token_u.clone());
        expr = expr.replace(&stack_place, value);
    } else if expr.contains("heap") {
        let cell_token_v: Vec<String> =
            them_tokenizer::double_tokenizer(expr_tokens.clone(), "heap(", ")");
        let cell_token_u = cell_token_v.get(0).cloned().unwrap_or_else(|| {
            println!("{}: No token provided", "Error".red());
            std::process::exit(1);
        });

        let cell_token_c: usize = cell_token_u.clone().parse().expect("Failed to parse");
        let cell_token_metadata: &String = &heap[cell_token_c];
        let parts: Vec<&str> = cell_token_metadata.split('.').collect();
        let value = parts.get(0).cloned().unwrap_or_else(|| {
            println!("{}: No token provided", "Error".red());
            std::process::exit(1);
        });

        let heap_place: String = format!("heap({})", cell_token_u.clone());
        expr = expr.replace(&heap_place, value);
    }

    return expr;
}

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
            if let Ok(number) = first_token.parse::<i64>() {
                let edited_full_str = finder(full_str, stack.clone(), heap.clone());
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
                            println!("{}: Unknown Memory Space: {}", "Error".red(), first_token);
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
