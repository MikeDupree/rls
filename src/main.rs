use std::env;

mod display;
mod options;
pub use display::print::*;
pub use options::command_options::*;

fn main() {
    let show_colors = false;
    if show_colors {
        println!("\x1b[{}m {}\x1b[0m", 90, 90);
        println!("\x1b[{}m {}\x1b[0m", 91, 91);
        println!("\x1b[{}m {}\x1b[0m", 92, 92);
        println!("\x1b[{}m {}\x1b[0m", 93, 93);
        println!("\x1b[{}m {}\x1b[0m", 94, 94);
        println!("\x1b[{}m {}\x1b[0m", 95, 95);
        println!("\x1b[{}m {}\x1b[0m", 96, 96);
        println!("\x1b[{}m {}\x1b[0m", 97, 97);
        println!("\x1b[{}m {}\x1b[0m", 98, 98);
        println!("\x1b[{}m {}\x1b[0m", 99, 99);
    }

    let opts = get_args();
    let filepath_glob = format!("{}{}", opts.filepath.as_str(), "*");

    is_git_dir();
    println!("> \x1b[90m{}\x1b[0m", env::current_dir().unwrap().display(),);
    if is_git_dir() {
        println!("\x1b[90m (\x1b[0m{}\x1b[90m )\x1b[0m", git_branch());
    }
    print_files(filepath_glob, opts);
}
