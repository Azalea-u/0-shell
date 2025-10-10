// Parses user input into Command structs

pub fn parse_input(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            '\\' => {
                if let Some(next) = chars.next() {
                    if in_quotes {
                        match next {
                            '"' | '$' | '`' | '\\' | '\n' => current.push(next),
                            _ => {
                                current.push('\\');
                                current.push(next);
                            }
                        }
                    } else {
                        current.push(next);
                    }
                } else {
                    current.push('\\');
                }
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}