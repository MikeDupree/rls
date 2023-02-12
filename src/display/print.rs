use glob::glob;
use prettytable::{format, row, Table};
use std::os::linux::fs::MetadataExt;
use std::path::PathBuf;

use crate::display::formatter;
use crate::display::utils;
use crate::options::command_options;

pub use command_options::*;
pub use formatter::*;
pub use utils::*;

pub fn print_file(path: PathBuf) {
    println!("{}", format_file_git_status(&path))
}

pub fn print_files(filepath_glob: String, opts: CommandOptions) {
    if opts.detailed {
        print_files_detailed(filepath_glob, opts);
        return;
    }

    if opts.recursive {
        print_files_recursive(filepath_glob, opts);
        return;
    }

    print_files_simple(filepath_glob, opts);
}

pub fn print_files_simple(filepath_glob: String, opts: CommandOptions) {
    let files = glob(filepath_glob.as_str()).expect("Failed to read glob pattern");
    let mut file_count = 0;
    let mut list_output = String::new();

    for entry in files {
        match entry {
            Ok(path) => {
                if !opts.show_hidden && is_hidden_file(&path) {
                    continue;
                }
                file_count += 1;
                list_output.push_str(format!("{}  ", format_file(&path)).as_str())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("\n{}\n\n {} files\n", list_output, file_count);
}

pub fn print_files_recursive(filepath_glob: String, opts: CommandOptions) {
    let current_level = 0;
    let max_depth = 3;
    println!(
        "{}",
        format_files_recursive(&filepath_glob, &current_level, &max_depth, &opts)
    );
}

pub fn print_files_detailed(filepath_glob: String, opts: CommandOptions) {
    println!("Detailed listing::");
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
        format_table_header("Modified", 90),
        format_table_header("User ID", 90),
        format_table_header("Size", 90),
    ]);

    println!("loop files::");
    // Create file table rows
    for file in files {
        match file {
            Ok(path) => {
                println!("-> file ::i is hidden");
                if !opts.show_hidden && is_hidden_file(&path) {
                    continue;
                }

                println!("build row output");
                file_count += 1;
                table.add_row(row![
                    format_file(&path),
                    format_git(&path, &opts),
                    format_permissions(&path),
                    format_time(path.metadata().unwrap().modified().unwrap()),
                    format_user_name(path.symlink_metadata().unwrap().st_gid()),
                    format_dir_size(dir_size(path.into_os_string()).unwrap_or_default()),
                ]);

                println!("done with file");
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("print");

    table.printstd();
    println!("Total: {}", file_count);
}
