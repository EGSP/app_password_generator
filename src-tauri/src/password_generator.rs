use rand::Rng;
use rand::seq::SliceRandom;
use tauri::api::dialog::message;
pub fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789~!@#$%^&*()_+-=[]{}|;:<>/?";
    (0..length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

const SPECIAL_SYMBOLS_EXTENDED: &str = "~!@#$%^&*()_+-=[]{}|;:<>/?";
const SPECIAL_SYMBOLS_BASIC: &str = "!@#$%^&*_?";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";

pub fn generate_password_extended(
    length: usize,
    special_symbols: bool,
    numbers: bool,
    uppercase: bool,
    lowercase: bool,
    window: tauri::Window
) -> String {
    let mut password = String::with_capacity(length);
    let mut charsets: Vec<CharSet> = Vec::new();

    // check if at least one of the arguments is true
    if !special_symbols && !numbers && !uppercase && !lowercase {
        //panic!("At least one of the arguments must be true");
        message(Some(&window),"Warning","At least one of the arguments must be true");
        return String::new();
    }

    if special_symbols {
        charsets.push(CharSet::new(SPECIAL_SYMBOLS_BASIC));
    }
    if numbers {
        charsets.push(CharSet::new(NUMBERS));
    }
    if uppercase {
        charsets.push(CharSet::new(UPPERCASE));
    }    
    if lowercase {
        charsets.push(CharSet::new(LOWERCASE));
    }

    // check if there are enough lenght to generate the password
    if charsets.len()>length {
        panic!("Not enough lenght to generate the password with the selected charsets");
    }

    // shuffle the charsets
    charsets.shuffle(&mut rand::thread_rng());

    // chunks the length by the number of charsets
    let password_chunk_length = length / charsets.len();
    // the rest of the length
    let rest_chunk_length = length % charsets.len();

    // generate the password
    for charset in charsets.iter() {
        for _ in 0..password_chunk_length {
            let idx = rand::thread_rng().gen_range(0..charset.charset.len());
            password.push(charset.charset.chars().nth(idx).unwrap());
        }
    }
    // generate the rest of the password
    for _ in 0..rest_chunk_length {
        let idx = rand::thread_rng().gen_range(0..charsets[0].charset.len());
        password.push(charsets[0].charset.chars().nth(idx).unwrap());
    }

    // shuffle the password
    let mut password_vec = password.chars().collect::<Vec<char>>();
    password_vec.shuffle(&mut rand::thread_rng());
    password = password_vec.iter().collect();

    password
}
pub struct CharSet {
    charset: String,
}

impl CharSet {
    pub fn new(charset: &str) -> Self {
        Self {
            charset: charset.to_string(),
        }
    }
}