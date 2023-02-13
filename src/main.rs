use std::env;

mod display;
mod options;
pub use display::print::*;
pub use options::command_options::*;

fn main() {
    let opts = get_args();
    let filepath_glob = format!("{}{}", opts.filepath.as_str(), "*");

    println!("> \x1b[90m{}\x1b[0m", env::current_dir().unwrap().display(),);
    if is_git_dir() {
        println!("\x1b[90m (\x1b[0m{}\x1b[90m )\x1b[0m", git_branch());
    }
    print_files(filepath_glob, opts);
}
