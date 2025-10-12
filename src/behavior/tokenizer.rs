// parses advanced operators
#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirects: Vec<Redirect>
}

#[derive(Debug)]
pub struct Redirect {
    pub operator: String,
    pub file: String
}

pub fn tokens_parser(tokens: Vec<String>) -> Result<Command, String> {
    let mut command = Command{
        name: String::new(),
        args: Vec::new(),
        redirects: Vec::new()
    };
    let mut iter = tokens.into_iter();

    while let Some(token) = iter.next() {
        match token.as_str() {
            ">" | ">>" | "<" => {
                if let Some(file) = iter.next() {
                    command.redirects.push(Redirect { operator: token, file: file });
                } else {
                    return Err("redirection without distination".to_string());
                }
            }

            _=>{
                if command.name.is_empty() {
                    command.name = token;
                } else {
                    command.args.push(token);
                }
            }
        }
    }
    if command.name.is_empty() {
        return Err("emty commnad".to_string());
    }

    Ok(command)
}
