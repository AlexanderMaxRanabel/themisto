use colored::*;
use std::process;

pub fn mov(
    args_list: Vec<String>,
    mut stack: Vec<String>,
    mut heap: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    //mov x :: int 12 1x0auto | stack
    //mov x :: int 12 2x034
    let (name_var, type_of_value, value, location) = (
        args_list.get(0).unwrap(),
        args_list.get(2).unwrap(),
        args_list.get(3).unwrap(),
        args_list.get(4).unwrap(),
    );

    let metadata = format!("{}.{}.{}", name_var, value, type_of_value);

    if location == "stack" {
        stack.push(metadata);
    } else if location.starts_with("2x") {
        let uninteger_cell = location.split("2x").nth(1);
        let cell: usize = uninteger_cell
            .expect("Unknown")
            .parse()
            .expect("Failed to parse");

        heap[cell] = metadata;
    } else {
        println!("{}: Unknown Location: {}", "Error".red(), location);
        process::exit(1);
    }

    (stack, heap)
}
