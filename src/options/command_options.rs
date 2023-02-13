use std::env;

use crate::is_git_dir;

#[derive(Debug)]
pub struct CommandOptions {
    pub filepath: String,
    pub options: String,
    pub option_flags: Vec<char>,
    pub show_hidden: bool,
    pub detailed: bool,
    pub recursive: bool,
    pub git: bool,
    pub size: bool,
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

    let option_flags = options.chars().collect::<Vec<char>>();

    CommandOptions {
        filepath,
        options: options.clone(),
        option_flags: option_flags.clone(),
        show_hidden: option_flags.clone().contains(&'a'),
        detailed: option_flags.clone().contains(&'l'),
        recursive: option_flags.clone().contains(&'r'),
        git: is_git_dir(),
        size: option_flags.clone().contains(&'s'),
    }
}
