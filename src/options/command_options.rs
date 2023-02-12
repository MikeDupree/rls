use std::env;

#[derive(Debug)]
pub struct CommandOptions {
    pub filepath: String,
    pub options: String,
    pub options_vec: Vec<char>,
}

pub fn get_args() -> CommandOptions {
    let mut args: Vec<String> = env::args().collect();
    let mut filepath = String::from("");
    let mut options = String::from("");
    if args.remove(0).len() > 0 {
        for arg in &args {
            if arg.contains("-") {
                let split_args: String = arg.split('-').collect();
                options = split_args;
            } else {
                filepath = String::from(arg) + "/";
            }
        }
    }

    CommandOptions {
        filepath,
        options: options.clone(),
        options_vec: options.chars().collect::<Vec<char>>(),
    }
}
