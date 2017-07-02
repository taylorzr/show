use std::env;
use std::process;
use std::io::Write;
use std::process::{Command,Output};

fn parse_input() -> String {
    match env::args().nth(1) {
        Some(path_string) => path_string,
        None              => {
            println!("You must supply an argument");
            process::exit(1)
        }
    }
}

fn select_application(path_string: &String) -> &'static str {
    let path = std::path::Path::new(&path_string);

    if path.is_dir() {
        writeln!(&mut std::io::stderr(), "Contents of directory '{}':", path_string).unwrap();
        "ls"
    } else if path.is_file() {
        writeln!(&mut std::io::stderr(), "Contents of file '{}':", path_string).unwrap();
        "cat"
    } else {
        writeln!(&mut std::io::stderr(), "show: {}: No such file or directory", path_string).unwrap();
        process::exit(1);
    }
}

fn build_command(application: &str, path_string: &String) -> Command {
    let mut command = Command::new(application);
    command.arg(path_string);
    command
}

fn run_command(mut command: Command) -> Output {
    match command.output() {
        Ok(output) => output,
        Err(error) => {
            writeln!(&mut std::io::stderr(), "show: error: {}", error).unwrap();
            process::exit(1);
        }
    }
}

fn main() {
    // TODO:
    // Use ? syntax: https://doc.rust-lang.org/std/result/#the--syntax

    let path_string = parse_input();
    let application = select_application(&path_string);
    let command     = build_command(&application, &path_string);
    let output      = run_command(command);

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
