use inquire::{InquireError, Select};
use std::{env, fs, process::Command};

fn main() {
    let project_dirs = project_dirs();
    let options: Vec<&str> = project_dirs.iter().map(|s| &**s).collect();

    let answer: Result<&str, InquireError> =
        Select::new("Which project do you want to open?", options).prompt();

    match answer {
        Ok(choice) => open_project(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

pub fn project_dirs() -> Vec<String> {
    let project_dir_env = "PROJECTS_PATH";
    match env::var(project_dir_env) {
        Ok(v) => println!("{}: {}", project_dir_env, v),
        Err(e) => panic!("${} is not set ({})", project_dir_env, e),
    }

    let project_path: String = env::var(project_dir_env).unwrap();

    println!("Reading projects from path: {}", project_path);
    let mut project_dirs = vec![String::from(""); 0];

    let paths = fs::read_dir(&project_path).unwrap();

    for path in paths {
        let dir_entry = path.unwrap();
        if dir_entry.path().is_file() {
            continue;
        }

        project_dirs.push(dir_entry.path().display().to_string());
    }

    project_dirs
}

pub fn open_project(project_dir: &str) {
    println!("Starting project {}", project_dir);

    Command::new("code")
        .arg(project_dir)
        .output()
        .expect(format!("Failed to open {} in VsCode", project_dir).as_str());
}
