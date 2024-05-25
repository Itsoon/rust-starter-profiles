use colored::*;
use indexmap::IndexMap;
use std::env;
use std::fs;
use std::process::exit;

mod parser;
use parser::file_parser;

mod launch_profile;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = check_args_validity(args);

    let home_dir = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("HOME environment variable is not set"),
    };

    let directory_path = format!("{}/.config/rsp/profiles/", home_dir);

    let entries = match fs::read_dir(&directory_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
            return;
        }
    };

    let mut available_profile = IndexMap::new();

    for (index, entry) in entries.enumerate() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
                continue;
            }
        };

        let file_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(os_string) => os_string.to_string_lossy().into_owned(),
        };

        available_profile.insert(index as i32, file_name);
    }

    if arg == "l" || arg == "ls" || arg == "list" {
        list_available_profile(available_profile);
        exit(0);
    } else if arg == "h" || arg == "-h" || arg == "-help" || arg == "--help" || arg == "help" {
        display_help();
        exit(0);
    } else if let Ok(arg_i32) = arg.parse::<i32>() {
        if available_profile.contains_key(&arg_i32) {
            file_parser(available_profile[&arg_i32].to_string());
        } else {
            print!("{}", "Number out of range\n".bright_red());
            display_help();
        }
    } else {
        println!("{}", "Bad character".bright_red());
        display_help();
        exit(0);
    }
}

fn check_args_validity(args: Vec<String>) -> String {
    if args.len() == 2 {
        args[1].clone()
    } else {
        display_help();
        exit(0);
    }
}

fn list_available_profile(available_profile: IndexMap<i32, String>) {
    for (index, value) in available_profile.iter() {
        println!("{}  {}", index, value);
    }
}

fn display_help() {
    let help: &str = "\
Usage: starter-profile [COMMAND]

Commands:
  l, ls, list   Print available profiles
  <profile>  Processing profile
  h, help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version";
    println!("{}", help);
}
