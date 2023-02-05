use glob::glob;
use glob::Paths;
use std::{fs::metadata, os::unix::prelude::PermissionsExt, path::PathBuf};

use prettytable::{format, row, Table};

fn main() {
    println!("Hello, world!");
    for entry in glob("*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => print_file(path),
            Err(e) => println!("{:?}", e),
        }
    }

    print_files_detailed(glob("*").expect("Failed to read glob pattern"));
}

fn print_file(path: PathBuf) {
    if path.is_dir() {
        println!(
            "\x1b[{}m 📁 {} {:?}\x1b[0m",
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
        return format!(
            "\x1b[{}m 📁 {} {:?}\x1b[0m",
            91,
            path.display(),
            path.metadata().unwrap().permissions().mode()
        );
    }

    return format!("\x1b[{}m  {}\x1b[0m", 93, path.display());
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

fn print_files_detailed(files: Paths) {
    let mut table = Table::new();
    //table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    table.set_titles(row![
        format_table_header("Name", 93),
        format_table_header("Permissions", 93),
    ]);

    // Create file table rows
    for file in files {
        match file {
            Ok(path) => {
                table.add_row(row![format_file(&path), format_permissions(&path)]);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    table.printstd();
}
