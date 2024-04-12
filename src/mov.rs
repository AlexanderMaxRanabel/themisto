use colored::*;
use std::{process};

pub fn mov(
    tokens: Vec<String>,
    mut stack: Vec<String>,
    mut heap: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    let mov_target = tokens.get(1).unwrap();
    let (name_var, type_of_value, value) = (
        tokens.get(2).unwrap(),
        tokens.get(3).unwrap(),
        tokens.get(4).unwrap(),
    );

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
    (stack, heap)
}
