use colored::*;
use std::process;

pub fn mov(
    tokens: Vec<String>,
    mut stack: Vec<String>,
    mut heap: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    //let x :: Int = 4 vm.heap
    let (name_var, type_of_value, value) = (
        tokens.get(1).unwrap(),
        tokens.get(3).unwrap(),
        tokens.get(5).unwrap(),
    );

    let metadata = format!("{}.{}.{}", name_var, value, type_of_value);
    if tokens.len() > 6 {
        let non_usize_cell = tokens.get(7).unwrap().to_string();
        let cell: usize = non_usize_cell.parse().expect("Failed to parse");

        heap[cell] = metadata;
    } else {
        stack.push(metadata);
    }
    (stack, heap)
}
