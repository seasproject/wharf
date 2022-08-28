pub fn tokenize_line(line: &str) -> (String, Vec<String>) {
    let chars = line.chars();
    let mut args: Vec<String> = Vec::new();
    let mut buffer = String::new();

    let mut in_str = false;
    let mut escaped = false;
    for char in chars {
        if escaped {
            match char {
                'n' => buffer.push('\n'),
                //'"' => buffer.push('"'),
                '\\' => buffer.push('\\'),
                _ => panic!("Unknown escape character {char}"),
            }
            escaped = false;
        } else if char.is_ascii_whitespace() && !in_str {
            if !buffer.is_empty() {
                args.push(buffer.to_string());
                buffer = String::new();
            }
        } else if char == '\\' && in_str {
            escaped = true;
        } else if char == '"' {
            // String
            in_str = !in_str;
        } else if char.is_ascii() {
            buffer.push(char);
        } else {
            panic!("Unexpected char {char}")
        }
    }

    if !buffer.is_empty() {
        args.push(buffer.to_string());
    }

    let args = args.split_first().unwrap();
    let instr: String = args.0.to_owned();
    let args = args.1;

    (instr, args.to_vec())
}
