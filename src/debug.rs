use colored::*;
use std::fs::File;
use std::io::prelude::*;

pub fn dumpstate(stack: Vec<String>, heap: Vec<String>) -> anyhow::Result<()> {
    let (line_stack, line_heap) = (stack.join(" "), heap.join(" "));

    Ok(())
}
