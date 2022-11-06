use std::env;

use prtpath::{EntryStatus, split_entries};

fn main() {
    let mut env_key = String::from("PATH");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        env_key = String::from(&args[1]);
    }
    let all_paths = split_entries(&env_key);
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