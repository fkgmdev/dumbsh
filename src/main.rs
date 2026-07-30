#![allow(clippy::all, unused)]
use std::{
    env::{current_dir, set_current_dir},
    fs::{FileType, read_dir},
    io::{Write, stdin, stdout},
    path::PathBuf,
};

struct Shell {
    cwd: PathBuf,
    prompt: String,
    history: Vec<String>,
}
impl Shell {
    fn new() -> Self {
        let current = current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        Self {
            cwd: current.clone(),
            prompt: format!("{} => ", current.display()),
            history: Vec::new(),
        }
    }
}

fn exec_command(command: &str, shell: &mut Shell) {
    let arguments: Vec<&str> = command.split_whitespace().collect();
    match arguments[0] {
        "cd" => {
            if arguments.len() != 2 {
                println!("Usage: cd <path>");
            } else {
                let target = arguments[1];
                if let Err(e) = set_current_dir(target) {
                    println!("cd failed: {}", e);
                } else {
                    if let Ok(new_cwd) = current_dir() {
                        shell.cwd = new_cwd;
                    }
                }
            }
        }
        "ls" => {
            if arguments.len() > 2 {
                println!("Usage: ls <path> (optional)");
            }
            else {
                let target_dir = if arguments.len() == 2 {
                    arguments[1]
                }
                else {
                    "."
                };
                if arguments.len() != 1 {
                    println!("takes no args lil bro");
                } else {
                    match read_dir(&shell.cwd) {
                        Ok(entries) => {
                            let mut output_string: Vec<String> = Vec::new();
                            for item in entries {
                                let entry = item.unwrap();
                                if entry.file_type().unwrap().is_dir() {
                                    output_string.push(format!(
                                        "Directory: {}",
                                        entry.file_name().to_string_lossy()
                                    ));
                                } else {
                                    output_string
                                        .push(format!("File: {}", entry.file_name().to_string_lossy()));
                                }
                            }
                            let display_string = output_string.join("\n");
                            println!("{}", display_string);
                        }
                        Err(e) => println!("error: {e}"),
                    }
                }
            }
        }
        _ => println!("invalid command"),
    }
}

fn main() {
    let mut shell = Shell::new();
    loop {
        print!("{} => ", shell.cwd.display());
        stdout().flush().unwrap();
        let mut raw_input = String::new();
        let _ = stdin().read_line(&mut raw_input);
        let command = raw_input.trim();
        shell.history.push(command.to_string());
        match command {
            "" => {}
            "pwd" => println!("{}", shell.cwd.to_str().unwrap()),
            "exit" => break,
            _ => exec_command(command, &mut shell),
        }
    }
}
