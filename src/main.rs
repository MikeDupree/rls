use glob::glob;
use glob::Paths;
use std::env;
use std::os::linux::fs::MetadataExt;
use std::time::SystemTime;
use std::{fs, io};
use std::{os::unix::prelude::PermissionsExt, path::PathBuf};

use prettytable::{format, row, Table};

fn main() {
    let opts = get_args();

    let files = glob(opts.filepath.as_str()).expect("Failed to read glob pattern");
    println!("Total: {}", files.count());

    if opts.options == String::from("") {
        // TODO make a grid output
        for entry in glob(opts.filepath.as_str()).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => print_file(path),
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        print_files_detailed(glob(opts.filepath.as_str()).expect("Failed to read glob pattern"));
    }
}

#[derive(Debug)]
struct CommandOptions {
    filepath: String,
    options: String,
}

fn get_args() -> CommandOptions {
    let mut args: Vec<String> = env::args().collect();
    let mut filepath = String::from("*");
    let mut options = String::from("");
    if args.remove(0).len() > 0 {
        for arg in &args {
            if arg.contains("-") {
                // is option
                let split_args: String = arg.split('-').collect();
                options = split_args;
            } else {
                filepath = String::from(arg);
            }
        }
        println!("filepath :: {:?}", filepath);
        println!("opts :: {:?}", options);
    }

    CommandOptions { filepath, options }
}
fn systemtime_strftime(dt: SystemTime, format: &str) -> String {
    format!("{:?}s ago", dt.elapsed().unwrap().as_secs())
}

fn print_file(path: PathBuf) {
    if path.is_dir() {
        println!(
            "\x1b[{}m  {} {:?}\x1b[0m",
            91,
            path.display(),
            path.metadata().unwrap().permissions().mode()
        )
    } else {
        println!("\x1b[{}m  {}\x1b[0m", 93, path.display())
    }
}

fn format_file(path: &PathBuf) -> String {
    if path.is_dir() {
        return format!("\x1b[{}m  {}\x1b[0m", 91, path.display(),);
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
    return format!("\x1b[{}m {} {}\x1b[0m", 93, file_icon, path.display());
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

fn print_files_detailed(files: Paths) {
    let mut table = Table::new();
    //table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new(' ', ' ', ' ', ' '),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    table.set_titles(row![
        format_table_header("Name", 93),
        format_table_header("Permissions", 93),
        format_table_header("Last modified", 93),
        format_table_header("User ID", 93),
        format_table_header("Size", 93),
    ]);

    // Create file table rows
    for file in files {
        match file {
            Ok(path) => {
                table.add_row(row![
                    format_file(&path),
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
}
