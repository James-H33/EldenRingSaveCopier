mod save_game;
mod character;

use save_game::{
    SLOT_START_INDEX,
    SLOT_LENGTH,
    SAVE_HEADER_START_INDEX,
    SAVE_HEADER_LENGTH,
    CHAR_ACTIVE_STATUS_START_INDEX,
    CHAR_NAME_LENGTH,
    CHAR_LEVEL_LOCATION,
    CHAR_PLAYED_START_INDEX
};

use character::Character;
use std::env;
use std::fs;


fn slot_start_index(save: &Character) -> i32 {
    return SLOT_START_INDEX + (save.index * 0x10) + (save.index * SLOT_LENGTH);
}

fn header_start_index(save: &Character) -> i32 {
    return SAVE_HEADER_START_INDEX + (save.index * SAVE_HEADER_LENGTH);
}

fn print_char_names(contents: &Vec<u8>) {
    for i in 0..10 {
        let name_bytes = contents
            .iter()
            .skip((SAVE_HEADER_START_INDEX + (i as i32 * SAVE_HEADER_LENGTH)) as usize)
            .take(CHAR_NAME_LENGTH as usize);


        let name = String::from_utf8(name_bytes.cloned().collect()).unwrap();

        println!("Char: {}", name);
    }
}

fn copy_character(src: &str, dest: &str) {
    let mut contents = fs::read(src).unwrap();
    let mut new_save = fs::read(dest).unwrap();

    let source_save = get_char(&contents, 0);
    let target_save = get_char(&contents, 1);

    let source_save_data = source_save.save_data.clone();
    let source_save_header = source_save.header_data.clone(); 

    let target_save_slot_start_index = slot_start_index(&target_save);
    let target_save_header_start_index = header_start_index(&target_save);

    for i in 0..SLOT_LENGTH {
        let target_index = (target_save_slot_start_index + i) as usize;

        new_save[target_index] = source_save_data[i as usize];
    }

    for i in 0..SAVE_HEADER_LENGTH {
        let target_index = (target_save_header_start_index + i) as usize;

        new_save[target_index] = source_save_header[i as usize];
    }

    fs::write("bak-target-ER0000.sl2", new_save).unwrap();
}

fn main() {
    let from_file_path = "ER0000.sl2";
    let to_file_path = "target-ER0000.sl2";

    copy_character(from_file_path, to_file_path);
}

fn get_char(bytes: &[u8], slot_index: i32) -> Character {
    let mut char = Character {
        index: slot_index,
        active: false,
        character_name: String::from(""),
        character_level: 0,
        seconds_played: 0,
        save_data: Vec::new(),
        header_data: Vec::new()
    };

    let is_active = bytes
        .iter()
        .skip(CHAR_ACTIVE_STATUS_START_INDEX as usize)
        .nth(slot_index as usize)
        .unwrap();

    char.active = *is_active == 1;


    let name_bytes = bytes
        .iter()
        .skip((SAVE_HEADER_START_INDEX + (slot_index as i32 * SAVE_HEADER_LENGTH)) as usize)
        .take(CHAR_NAME_LENGTH as usize);

    let char_name = String::from_utf8(name_bytes.cloned().collect()).unwrap();
    char.character_name = char_name;


    let level_bytes = bytes
        .iter()
        .skip((SAVE_HEADER_START_INDEX + (slot_index as i32 * SAVE_HEADER_LENGTH)) as usize)
        .nth(CHAR_LEVEL_LOCATION as usize)
        .unwrap();

    let level = level_bytes.clone() as u16;
    char.character_level = level;


    let skip_for_seconds = SAVE_HEADER_START_INDEX + (slot_index * SAVE_HEADER_LENGTH) + CHAR_PLAYED_START_INDEX;
    let second_bytes = bytes
        .iter()
        .skip(skip_for_seconds as usize)
        .take(4);

    /*
        Explaination for my_seconds.try_into()
        my_seconds is a Vec<u8> and when we call .try_into we are asking
        the rust compiler to look at the type we might want to covert 
        my_seconds into based on the context. The context in this case
        is the input type of u32::from_le_bytes which is an array of u8 
        with a size captured at runtime.
    */ 
    let my_seconds: Vec<u8> = second_bytes.cloned().collect();
    let seconds: u32 = u32::from_le_bytes(my_seconds.try_into().unwrap());
    char.seconds_played = seconds;


    let skip_for_save = SLOT_START_INDEX + (slot_index * 0x10) + (slot_index * SLOT_LENGTH);
    let saved_btyes = bytes
        .iter()
        .skip(skip_for_save as usize)
        .take(SLOT_LENGTH as usize);

    char.save_data = saved_btyes.cloned().collect();

    let skip_for_header = SAVE_HEADER_START_INDEX + (slot_index * SAVE_HEADER_LENGTH);
    let header_bytes = bytes
        .iter()
        .skip(skip_for_header as usize)
        .take(SAVE_HEADER_LENGTH as usize);

    char.header_data = header_bytes.cloned().collect();

    char
}

fn map_name(src: &mut Vec<u8>) {
    let mut all_name_bytes: Vec<std::iter::Take<std::iter::Skip<std::slice::Iter<u8>>>> = vec![];

    for i in 0..10 {
        let name_bytes = src
            .iter()
            .skip((SAVE_HEADER_START_INDEX + (i as i32 * SAVE_HEADER_LENGTH)) as usize)
            .take(CHAR_NAME_LENGTH as usize);

        all_name_bytes.push(name_bytes.clone());
    }

    let mut k = 0;
    let mut i = SAVE_HEADER_START_INDEX + (1 as i32 * SAVE_HEADER_LENGTH); 
    let end = i + CHAR_NAME_LENGTH;
    let first_name: Vec<u8> = all_name_bytes[0].clone().cloned().collect();

    while i < end {
        src[i as usize] = first_name[k as usize];

        i += 1;
        k += 1;
    }
}
