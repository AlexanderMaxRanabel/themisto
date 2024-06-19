use crate::{mem_manage, mov, printline};
use colored::*;
use std::process;

#[allow(unused_assignments)]

pub fn themisto_runtime(
    main_keyword: String,
    tokens: Vec<String>,
    args_list: Vec<String>,
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

        "printline" => {
            (stack, heap) = printline::println(args_list, stack, heap);
        }

        "pop_stack" => {
            stack = mem_manage::pop_stack(stack);
        }

        "pop_heap" => {
            heap = mem_manage::pop_heap(heap);
        }

        _ => {
            println!("{}: Unknown keyword: {}", "Error".red(), main_keyword);
            process::exit(1);
        }
    }
    
    (stack, heap)
}
