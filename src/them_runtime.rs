use crate::{mem_manage, mov};
use colored::*;
use std::process;

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
            (stack, heap) = mov::mov(tokens, stack, heap);
        }

        _ => {
            println!("{}: Unknown keyword: {}", "Error".red(), main_keyword);
            process::exit(1);
        }
    }

    (stack, heap)
}
