#[derive(Debug)]
pub struct Character {
    pub index: i32,
    pub active: bool,
    pub character_name: String,
    pub character_level: u16,
    pub seconds_played: u32,
    pub save_data: Vec<u8>,
    pub header_data: Vec<u8>
}