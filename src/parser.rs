use crate::{them_runtime, them_tokenizer};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parser(file: String, mut stack: Vec<String>, mut heap: Vec<String>) -> anyhow::Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut function_bodies: Vec<String> = Vec::new();
    let mut function_ids: Vec<String> = Vec::new();

    let re = Regex::new(r"func(.*?)end").unwrap();

    let function_bodies: Vec<String> = them_tokenizer::double_tokenizer();

    for mat in re.find_iter(contents) {
        let portion = mat.as_str().trim().to_string();
        function_bodies.push(portion);
    }

    for line in reader.lines() {
        let line = line?;
        let tokens = them_tokenizer::linear_tokenizer(line.clone());
        let main_keyword = tokens.get(0).cloned().unwrap_or_else(|| {
            std::process::exit(1);
        });

        if main_keyword == "func" {
            let function_name = tokens.get(1).cloned().unwrap_or_else(|| {
                std::process::exit(1);
            });

            let function_param_list: Vec<String> =
                them_tokenizer::double_tokenizer(tokens, "(", ")");
        }

        (stack, heap) = them_runtime::themisto_runtime(main_keyword, tokens, stack, heap);
    }

    Ok(())
}
