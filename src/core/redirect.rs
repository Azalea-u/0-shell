// redirection handeler

use std::io::{self, Write, Read};
use crate::behavior::tokenizer::Redirect;

pub fn get_output_stream(redirects: &[Redirect]) -> io::Result<Box<dyn Write>> {
    if let Some(redirect) = redirects.iter().find(|r| r.operator == ">" || r.operator == ">>") {
        match redirect.operator.as_str() {
            ">" => Ok(Box::new(std::fs::File::create(&redirect.file)?)),
            ">>" => Ok(Box::new(
                std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&redirect.file)?
            )),
            _ => Ok(Box::new(io::stdout())),
        }
    } else {
        Ok(Box::new(io::stdout()))
    }
}

pub fn get_input_stream(redirects: &[Redirect]) -> io::Result<Box<dyn Read>> {
    if let Some(redirect) = redirects.iter().find(|r| r.operator == "<") {
        Ok(Box::new(std::fs::File::open(&redirect.file)?))
    } else {
        Ok(Box::new(io::stdin()))
    }
}
