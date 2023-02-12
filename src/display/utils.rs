use std::process::Command;
use std::path::PathBuf;

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

pub fn git_branch() -> String {
    let output = Command::new("/usr/bin/git")
        .arg("branch")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    println!("branch name {:?}", &output.stdout);
    println!("branch name {:?}", String::from_utf8_lossy(&output.stdout));
    format!(
        "{}",
        String::from_utf8_lossy(&output.stdout)
            .replace("* ", ">")
            .replace("\n", "")
    )
}
