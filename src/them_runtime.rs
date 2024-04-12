use colored::*;
use std::{process};
use crate::{mem_manage, them_tokenizer};

pub fn themisto_runtime(
    main_keyword: String,
    tokens: Vec<String>,
    mut stack: Vec<String>,
    mut heap: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    match main_keyword.as_str() {
        "set_heap" => {
            let non_usize_range = tokens.get(1).cloned().unwrap_or_else(|| {
                println!(
                    "{}: No range for allocation of memory provided",
                    "Error".red()
                );
                std::process::exit(1);
            });

            let range: usize = non_usize_range.parse().expect("Couldnt parse");

            heap.resize(range, "".to_string());
        }

        "reset_stack" => {
           stack = mem_manage::reset_stack(stack);
        }

        "reset_heap" => {
            heap = mem_manage::reset_heap(heap);
        }

        "mov" => {
            let mov_target = tokens.get(1).unwrap();
            let (name_var, type_of_value, value) = (tokens.get(2).unwrap(), tokens.get(3).unwrap(), tokens.get(4).unwrap());
            
            let metadata = format!("{}.{}.{}", name_var, value, type_of_value);
            match mov_target.as_str() {
                "vm.stack" => {
                    stack.push(metadata);
                }

                "vm.heap" => {
                    let non_usize_cell = tokens.get(5).unwrap().to_string();
                    let cell: usize = non_usize_cell.parse().expect("Failed to parse");

                    heap[cell] = metadata;
                }

                _ => {
                    println!("{}: Unknown in-mem vector: {}", "Error".red(), mov_target);   
                    process::exit(1);
                }
            }
        }

        _ => {
            println!("{}: Unknown keyword: {}", "Error".red(), main_keyword);
            process::exit(1);
        }
    }

    (stack, heap)
}
