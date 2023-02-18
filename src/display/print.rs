use glob::glob;
use prettytable::{format, row, Table};
use prettytable::{Cell, Row};
use std::os::linux::fs::MetadataExt;
use std::path::PathBuf;

use crate::display::console;
use crate::display::formatter;
use crate::display::utils;
use crate::options::command_options;

pub use command_options::*;
pub use console::*;
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
    let console_size = get_console_size();

    let mut file_count = 0;
    let mut list_output = String::new();
    let mut line_len = 0;

    for entry in files {
        match entry {
            Ok(path) => {
                if !opts.show_hidden && is_hidden_file(&path) {
                    continue;
                }

                file_count += 1;

                let file_output = format_file(&path);
                let mut prefix = "";

                if line_len + file_output.len() > console_size.0 as usize {
                    prefix = "\n";
                    line_len = 0;
                }

                let formatted_output = format!("{}{}  ", prefix, file_output);
                line_len += formatted_output.len();
                list_output.push_str(formatted_output.as_str())
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

    let mut table_header = vec![
        Cell::new(format_table_header("Name", 90).as_str()),
        Cell::new(format_table_header("Git Status", 90).as_str()),
        Cell::new(format_table_header("Permissions", 90).as_str()),
        Cell::new(format_table_header("Modified", 90).as_str()),
        Cell::new(format_table_header("User", 90).as_str()),
    ];

    if opts.size {
        table_header.push(Cell::new(format_table_header("Size", 90).as_str()));
    }

    table.set_titles(Row::new(table_header));

    // Create file table rows
    for file in files {
        match file {
            Ok(path) => {
                if !opts.show_hidden && is_hidden_file(&path) {
                    continue;
                }

                file_count += 1;
                let mut table_row = vec![
                    Cell::new(format_file(&path).as_str()),
                    Cell::new(format_git(&path, &opts).as_str()),
                    Cell::new(format_permissions(&path).as_str()),
                    Cell::new(format_time(path.metadata().unwrap().modified().unwrap()).as_str()),
                    Cell::new(format_user_name(path.symlink_metadata().unwrap().st_gid()).as_str()),
                ];
                if opts.size {
                    table_row.push(Cell::new(
                        format_dir_size(dir_size(path.into_os_string()).unwrap_or_default())
                            .as_str(),
                    ));
                }
                table.add_row(Row::new(table_row));
            }
            Err(e) => println!("{:?}", e),
        }
    }

    table.printstd();
    println!("Total: {}", file_count);
}
