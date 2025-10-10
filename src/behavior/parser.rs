// Parses user input into Command structs

pub fn parse_input(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            '\\' => {
                if let Some(&next) = chars.peek() {
                    match next {
                        _ if in_single_quotes => {
                            current.push('\\');
                            current.push(next);
                            chars.next();
                        }
                        '"' | '\\' | '$' | '`' | '\n' if in_double_quotes => {
                            current.push(next);
                            chars.next();
                        }
                        _ if !in_single_quotes && !in_double_quotes => {
                            current.push(next);
                            chars.next();
                        }
                        _ => {
                            current.push('\\');
                        }
                    }
                } else {
                    current.push('\\');
                }
            }

            ' ' | '\t' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    tokens.push(current);
                    current = String::new();
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

pub fn gate_closed(input: &str) -> bool {
    let mut in_single = false;
    let mut in_double = false;
    let mut escape = false;

    for c in input.chars() {
        if escape {
            escape = false;
            continue;
        }

        match c {
            '\\' => {
                escape = true;
            }
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            _ => {}
        }
    }

    // All quotes must be closed
    !in_single && !in_double
}