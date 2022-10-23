use std::env;
use std::path::PathBuf;
use std::path::is_separator;

enum EntryStatus {
    INVALID,
    DUPLICATE,
    DIRECTORY,
    FILE
}

struct Entry {
    path: String,
    status: EntryStatus,
}

fn return_status(path: &PathBuf) -> EntryStatus {
    let mut status = EntryStatus::INVALID;
    if path.is_dir() {
        status = EntryStatus::DIRECTORY;
    }
    if path.is_file() {
        status = EntryStatus::FILE;
    }
    status
}

fn path_buff_to_string(path_buff: &PathBuf) -> String {
    match path_buff.to_str() {
        Some(path_str) => String::from(path_str),
        None => String::from("")
    }
}

fn remove_trailing_separator_from_string(path_str: &String) -> String {
    let mut result = path_str.to_string();
    let last_char: char = match path_str.chars().last() {
        Some(c) => c,
        None => 'x',
    };
    if is_separator(last_char) {
        result.pop();
    }
    result
}

fn remove_trailing_separator_from_path(path: &PathBuf) -> String {
    let path_str = path_buff_to_string(path);
    remove_trailing_separator_from_string(&path_str)
}

fn check_dups(path: &PathBuf, path_list: &Vec<Entry>) -> bool {
    let mut dup = false;
    for entry in path_list {
        let path_str = remove_trailing_separator_from_path(path);
        if path_str == remove_trailing_separator_from_string(&entry.path) {
            dup = true;
            break;
        }
    }
    dup
}

fn main() {
    let mut env_key = "PATH";
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        env_key = &args[1];
    }
    let mut all_paths = Vec::new();
    match env::var_os(env_key) {
        Some(paths) => {
            for path_buff in env::split_paths(&paths) {
                let mut entry = Entry {
                    path: path_buff_to_string(&path_buff),
                    status: return_status(&path_buff)
                };
                if check_dups(&path_buff, &all_paths) {
                    entry.status = EntryStatus::DUPLICATE;
                }
                all_paths.push(entry);
            }
        },
        None => println!("{env_key} is not defined in the environment.")
    }
    let mut idx: usize = 0;
    println!("{}", "Index Status    Path");
    for path_item in all_paths.iter() {
        idx += 1;
        let status_str = match path_item.status {
            EntryStatus::INVALID   => "-invalid-",
            EntryStatus::DUPLICATE => "-dup-----",
            EntryStatus::DIRECTORY => " dir",
            EntryStatus::FILE      => " file",
        };
        println!("{:4}  {:9} {}", idx, status_str, path_item.path);
    }
    println!("Found {} path entries", all_paths.len())
}