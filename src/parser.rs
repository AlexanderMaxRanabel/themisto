use crate::{them_runtime, them_tokenizer};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parser(file: String, mut stack: Vec<String>, mut heap: Vec<String>) -> anyhow::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let tokens = them_tokenizer::linear_tokenizer(line.clone());
        let main_keyword = tokens.get(0).cloned().unwrap_or_else(|| {
            std::process::exit(1);
        });

        (stack, heap) = them_runtime::themisto_runtime(main_keyword, tokens, stack, heap);
    } 

    Ok(())
}
