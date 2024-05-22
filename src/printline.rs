use crate::opcodes;
use colored::*;

pub fn println(mut tokens: Vec<String>) -> anyhow::Result<()> {
    tokens.remove(0);
    let full_str = tokens.join(" ");
    if full_str.starts_with("'") && full_str.ends_with("'") {
        println!("{}", full_str);
    } else {
        tokens.get(0).unwrap();
        unimplemented!()
    }
    Ok(())
}
