use std::fmt::format;
use std::path::PathBuf;
use std::process::Command;

pub fn is_hidden_file(path: &PathBuf) -> bool {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .chars()
        .next()
        .unwrap()
        == '.'
}

pub fn is_git_dir() -> bool {
    let output = Command::new("/usr/bin/git")
        .arg("branch")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).len() > 0
}

pub fn git_branch() -> String {
    let output = Command::new("/usr/bin/git")
        .arg("branch")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    let raw_output = String::from_utf8_lossy(&output.stdout)
        .replace("* ", "*")
        .replace("\n", "");
    let branches: Vec<&str> = raw_output.split(" ").collect();

    let mut output = String::new();
    for branch in branches {
        let mut color = 90;
        if branch.contains("*") {
            color = 92;
        }

        output.push_str(format!("\x1b[{}m  {}  \x1b[0m", color, branch.replace("*", "")).as_str())
    }
    format!("{}", output)
}
