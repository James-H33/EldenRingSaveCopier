mod save_game;
mod character;

use character::print_char_names;
use character::{
    copy_character
};

use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct CharArgs {
    from_file_path: String,
    to_file_path: String,
    from_char_index: i32,
    to_char_index: i32
}

impl CharArgs {
    pub fn new(args: Vec<String>) -> Self {
        let mut from = args[1].clone();
        let from_char = args[2].clone();
        let mut to = args[3].clone();
        let to_char = args[4].clone();

        from = parse_arg(from);
        let from_char_index: i32 = parse_arg(from_char).parse().unwrap();

        to = parse_arg(to);
        let to_char_index: i32 = parse_arg(to_char).parse().unwrap();

        Self {
            from_file_path: from,
            to_file_path: to,
            from_char_index: from_char_index - 1,
            to_char_index: to_char_index - 1
        }
    }
}

fn parse_arg(s: String) -> String  {
    let mut value = "";
    let mut i = 0;

    for char in s.chars() {
        if char == '=' {
            let start  = i + 1;
            value = &s[start..s.len()];
            break;
        }

        i += 1;
    }

    let res = value.to_string();

    res
} 

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Not enough arguments");
        println!("Format from=PATH_TO_FILE character=[1 - 10] to=PATH_TO_FILE character=[1 - 10]");
        process::exit(0x0100);
    }

    let char_args = CharArgs::new(args);

    copy_character(
        &char_args.from_file_path, 
        &char_args.from_char_index,
        &char_args.to_file_path,
         &char_args.to_char_index
    );


    let contents = fs::read(char_args.to_file_path).unwrap();
    print_char_names(&contents);
}

// [elden-ring-save-copier] cargo run from=ER0000.sl2 character=1 to=target-ER0000.sl2 character-7                                                        master  ✭