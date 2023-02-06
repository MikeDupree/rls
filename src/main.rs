use glob::glob;
use std::env;
use std::os::linux::fs::MetadataExt;
use std::process::Command;
use std::time::SystemTime;
use std::{fs, io};
use std::{os::unix::prelude::PermissionsExt, path::PathBuf};

use prettytable::{format, row, Table};

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

    println!(
        "{} \x1b[96m ({})\x1b[0m",
        env::current_dir().unwrap().display(),
        git_branch()
    );
    print_files(filepath_glob, opts);
}

#[derive(Debug)]
struct CommandOptions {
    filepath: String,
    options: String,
}

fn get_args() -> CommandOptions {
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

    CommandOptions { filepath, options }
}

fn git_branch() -> String {
    let output = Command::new("/usr/bin/git")
        .arg("branch")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    format!(
        "{}",
        String::from_utf8_lossy(&output.stdout)
            .replace("* ", "")
            .replace("\n", "")
    )
}

fn format_git(path: &PathBuf) -> String {
    let output = Command::new("/usr/bin/git")
        .arg("status")
        .arg(path.file_name().unwrap())
        .arg("-s")
        .output()
        .expect("failed to execute process");

    if output.stdout.len() > 0 {
        if String::from_utf8_lossy(&output.stdout)
            .contains(path.file_name().unwrap().to_str().unwrap())
        {
            let git_file_status = String::from_utf8_lossy(&output.stdout);
            let git_symbol = git_file_status.trim_start().chars().nth(0).unwrap();
            match git_symbol {
                '?' => return format!("\x1b[91m  Unstaged\x1b[0m"),
                'M' => return format!("\x1b[93m  Modified\x1b[0m"),
                'A' => return format!("\x1b[94m  Staged\x1b[0m"),
                _ => return format!("{}", ""),
            }
        }
    }
    format!("{}", "")
}

fn format_file_git_status(path: &PathBuf) -> String {
    let output = Command::new("/usr/bin/git")
        .arg("status")
        .arg(path.file_name().unwrap())
        .arg("-s")
        .output()
        .expect("failed to execute process");

    if output.stdout.len() > 0 {
        if String::from_utf8_lossy(&output.stdout)
            .contains(path.file_name().unwrap().to_str().unwrap())
        {
            let git_file_status = String::from_utf8_lossy(&output.stdout);
            let git_symbol = git_file_status.trim_start().chars().nth(0).unwrap();
            match git_symbol {
                '?' => return format!("{} \x1b[91m \x1b[0m", format_file(path)),
                'M' => return format!("{} \x1b[93m \x1b[0m", format_file(path)),
                'A' => return format!("{} \x1b[94m \x1b[0m", format_file(path)),
                _ => return format!("{}", format_file(path)),
            }
        }
    }
    format!("{}", format_file(path))
}

fn systemtime_strftime(dt: SystemTime, _format: &str) -> String {
    format!("\x1b[92m{}\x1b[0ms ago", dt.elapsed().unwrap().as_secs())
}

fn print_file(path: PathBuf) {
    println!("{}", format_file_git_status(&path))
}

fn format_file(path: &PathBuf) -> String {
    if path.is_dir() {
        return format!("\x1b[{}m  \x1b[0m{}", 93, path.display(),);
    }

    let mut file_icon = "";
    if path.extension().is_some() {
        match path.extension().unwrap().to_os_string().to_str().unwrap() {
            "lock" => file_icon = "",
            "toml" => file_icon = "",
            "md" => file_icon = "",
            "js" => file_icon = "",
            "ts" => file_icon = "",
            "rs" => file_icon = "",
            _ => (),
        }
    }
    return format!("\x1b[{}m {} \x1b[0m{}", 92, file_icon, path.display());
}

fn format_permissions(path: &PathBuf) -> String {
    return format!(
        "\x1b[{}m {:?}\x1b[0m",
        91,
        path.metadata().unwrap().permissions().mode()
    );
}

fn format_table_header(label: &str, color_code: u16) -> String {
    format!("\x1b[{}m{}\x1b[0m", color_code, label)
}

fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
        dir.try_fold(0, |acc, file| {
            let file = file?;
            let size = match file.metadata()? {
                data if data.is_dir() => dir_size(fs::read_dir(file.path())?)?,
                data => data.len(),
            };
            Ok(acc + size)
        })
    }

    dir_size(fs::read_dir(path.into())?)
}

fn print_files(filepath_glob: String, opts: CommandOptions) {
    println!("selected Opts: {}", opts.options);
    if opts.options == String::from("") {
        print_files_simple(filepath_glob);
        return;
    }

    if opts.options == String::from("r") {
        println!("Running Recursive Listing");
        print_files_recursive(filepath_glob);
        return;
    }

    print_files_detailed(filepath_glob);
}

fn print_files_simple(filepath_glob: String) {
    let files = glob(filepath_glob.as_str()).expect("Failed to read glob pattern");
    let mut file_count = 0;
    let mut list_output = String::new();

    for entry in files {
        match entry {
            Ok(path) => {
                file_count += 1;
                list_output.push_str(format!("{}  ", format_file(&path)).as_str())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("{}\n {} files", list_output, file_count);
}

fn format_files_recursive(filepath_glob: &String, level: &u16, max_depth: &u16) -> String {
    if level >= max_depth {
        return String::new();
    }

    let files = glob(filepath_glob.as_str()).expect("Failed to read glob pattern");
    let mut file_count = 0;
    let mut list_output = String::new();

    for entry in files {
        match entry {
            Ok(path) => {
                file_count += 1;
                list_output.push_str(format!("- {}\n", format_file(&path)).as_str());
                if !path.is_dir() {
                    continue;
                }

                let next_level = level + 1;
                list_output.push_str(
                    format_files_recursive(format!("{}", &next_level, max_depth).as_str(),
                );
            }
            Err(e) => println!("{:?}", e),
        }
    }

    format!("{} {}\n", level, list_output)
}

fn print_files_recursive(filepath_glob: String) {
    let current_level = 0;
    let max_depth = 3;
    println!(
        "{}",
        format_files_recursive(&filepath_glob, &current_level, &max_depth)
    );
}

fn print_files_detailed(filepath_glob: String) {
    let files = glob(filepath_glob.as_str()).expect("Failed to read glob pattern");
    let mut file_count = 0;
    let mut table = Table::new();
    //table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let format = format::FormatBuilder::new()
        .column_separator('┊')
        .borders('┊')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('┄', '┄', '❖', '❖'),
        )
        .padding(1, 1)
        .build();

    table.set_format(format);

    table.set_titles(row![
        format_table_header("Name", 90),
        format_table_header("Git Status", 90),
        format_table_header("Permissions", 90),
        format_table_header("Last modified", 90),
        format_table_header("User ID", 90),
        format_table_header("Size", 90),
    ]);

    // Create file table rows
    for file in files {
        match file {
            Ok(path) => {
                file_count += 1;
                table.add_row(row![
                    format_file(&path),
                    format_git(&path),
                    format_permissions(&path),
                    format!(
                        "{}",
                        systemtime_strftime(
                            path.metadata().unwrap().modified().unwrap(),
                            "%d/%m/%Y %T"
                        )
                    ),
                    format!("{:?}", path.symlink_metadata().unwrap().st_gid()),
                    format!("{:?}", dir_size(path.into_os_string()).unwrap_or_default()),
                ]);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    table.printstd();
    println!("Total: {}", file_count);
}
