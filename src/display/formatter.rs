/*
 * Formatters
 */
use std::process::Command;
use std::time::SystemTime;
use std::{fs, io};
use std::{os::unix::prelude::PermissionsExt, path::PathBuf};
use users::get_user_by_uid;

pub fn format_git(path: &PathBuf) -> String {
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

pub fn format_user_name(uid: u32) -> String {
    format!(
        "\x1b[96m{}\x1b[0m",
        get_user_by_uid(uid).unwrap().name().to_string_lossy()
    )
}

pub fn format_file_git_status(path: &PathBuf) -> String {
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

pub fn format_time(dt: SystemTime) -> String {
    // TODO handle remainder
    let mut time_elapsed = dt.elapsed().unwrap().as_secs();
    let mut time_symbol = "s";

    if time_elapsed > 60 {
        time_elapsed = time_elapsed / 60;
        time_symbol = " min";
    }

    if time_elapsed > 60 {
        time_elapsed = time_elapsed / 60;
        time_symbol = " hour";
    }

    if time_elapsed > 24 {
        time_elapsed = time_elapsed / 24;
        time_symbol = " days";
    }

    if time_elapsed > 7 {
        time_elapsed = time_elapsed / 7;
        time_symbol = " weeks";
    }

    format!(
        "\x1b[92m{}\x1b[0m\x1b[95m{}\x1b[0m",
        time_elapsed, time_symbol
    )
}

pub fn format_file(path: &PathBuf) -> String {
    if path.is_dir() {
        return format!(
            "\x1b[{}m  \x1b[0m{}",
            93,
            path.file_name().unwrap().to_str().unwrap()
        );
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
    return format!(
        "\x1b[{}m {} \x1b[0m{}",
        92,
        file_icon,
        path.file_name().unwrap().to_str().unwrap()
    );
}

fn get_permission_group_string(p: &char) -> String {
    match p {
        '0' => return String::from("---"),
        '1' => return String::from("--x"),
        '2' => return String::from("-w-"),
        '3' => return String::from("-wx"),
        '4' => return String::from("r--"),
        '5' => return String::from("r-x"),
        '6' => return String::from("rw-"),
        '7' => return String::from("rwx"),
        _ => return String::new(),
    }
}

pub fn format_permissions(path: &PathBuf) -> String {
    let mode = path.metadata().unwrap().permissions().mode();
    let mode_string = format!("{mode:o}");
    let permission_vec = format!("{}", mode_string[mode_string.len() - 3..].to_string())
        .chars()
        .collect::<Vec<char>>();

    let mut permissions: String = String::from("|");

    let mut color = 94;
    for p in &permission_vec {
        permissions.push_str(
            format!("\x1b[{}m{}\x1b[0m|", color, get_permission_group_string(p)).as_str(),
        );
        color += 1;
    }

    permissions
}

pub fn format_table_header(label: &str, color_code: u16) -> String {
    format!("\x1b[{}m{}\x1b[0m", color_code, label)
}

pub fn dir_size(path: impl Into<PathBuf>) -> io::Result<u64> {
    pub fn dir_size(mut dir: fs::ReadDir) -> io::Result<u64> {
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

pub fn format_dir_size(size: u64) -> String {
    // TODO show size with remainder
    let mut size_symbol = "b";
    let mut size_formatted = size;
    if size > 1000 {
        size_symbol = "kb";
        size_formatted = size / 1000;
    }

    if size > 1000000 {
        size_symbol = "mb";
        size_formatted = size / 1000000;
    }

    if size > 1000000000 {
        size_symbol = "GB";
        size_formatted = size / 1000000000;
    }

    format!(
        "\x1b[95m{}\x1b[0m\x1b[97m{}\x1b[0m",
        size_formatted, size_symbol
    )
}
