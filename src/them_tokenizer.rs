pub fn linear_tokenizer(line: String) -> Vec<String> {
    let str_tokens: Vec<&str> = line.split_whitespace().collect();
    let tokens = str_tokens.iter().map(|&s| s.to_owned()).collect();
    return tokens;
}

pub fn double_tokenizer(tokens: Vec<String>, starter: &str, ender: &str) -> Vec<String> {
    let raw_tokens: Vec<_> = tokens
        .iter()
        .skip_while(|c| c != &&starter)
        .skip(1)
        .take_while(|c| c != &&ender)
        .collect();

    let extracted_tokens: Vec<String> = raw_tokens.iter().map(|&s| s.to_string()).collect();

    return extracted_tokens;
}
