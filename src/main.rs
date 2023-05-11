mod words;
use std::io::{stdout, Write};
use std::io::stdin;
use rand::{ self, Rng };
use crate::words::words::WORDS;

enum GameState {
    MENU, INGAME, END, QUIT
}

struct GameData {
    avail_characters : [bool; 26],
    lives : u8,
    guess_word : String,
    progress: String,
    win: bool,
}

impl Default for GameData {
    fn default() -> Self {
        GameData { 
            avail_characters: [true; 26], 
            lives: 6,
            guess_word: String::new(),
            progress: String::new(),
            win: false,
        }
    }
}

fn init_game_data(data: &mut GameData) {
    *data = GameData::default(); 
    data.guess_word = get_random_word();
    data.progress = std::iter::repeat('_').take(data.guess_word.len()).collect::<String>()
}

fn get_random_word() -> String{
    WORDS[rand::thread_rng().gen_range(0..WORDS.len())].to_string()
}

fn scan_char_lower() -> char{
   let mut str: String = String::new();
    match stdin().read_line(&mut str) {
        Ok(_) => {}
        Err(_) => str = " ".to_string(),
    }
    str.bytes().nth(0).unwrap().to_ascii_lowercase() as char
}

fn main() {
    let mut state = GameState::MENU;
    let mut data: GameData = GameData::default();

    while !matches!(state, GameState::QUIT) {
        // clear screen
        print!("{esc}c", esc = 27 as char);

        println!("\t\t-------- HANGMAN --------\n\n");

        match state {
            GameState::MENU => menu(&mut state, &mut data),
            GameState::INGAME => game(&mut state, &mut data),
            GameState::END => end(&mut state, &mut data),
            _ => {},
        }
    }
}

fn menu(state: &mut GameState, data: &mut GameData) {
    println!("\t1. PLAY");
    println!("\t2. QUIT");
    println!("\n");

    print!("\tChoice: ");
    stdout().flush().unwrap();
    let choice: char = scan_char_lower();

    match choice {
        '1' => { 
            *state = GameState::INGAME; 
            init_game_data(data);
        },
        '2' => *state = GameState::QUIT,
        'q' => *state = GameState::QUIT,
        _ => {},
    }
}

fn game(state: &mut GameState, data: &mut GameData) {
    if data.lives == 0 {
        *state = GameState::END;
        return;
    }

    display_man(data.lives);
    println!();

    display_guess(&mut data.progress);
    display_avail(&mut data.avail_characters);

    print!("\t\tCharacter: ");
    stdout().flush().unwrap();
    let choice: char = scan_char_lower();

    process_guess(data, choice);
    check_status(state, data);
}

fn end(state: &mut GameState, data: &mut GameData) {
    if data.win {
        println!("\n\t\t\tYOU WIN!\n");
        println!("\n\t\t\tWord: {}\n", data.guess_word);
    }
    else {
        println!("\t\t\tYOU LOSE!\n");
        display_man(0);
        println!("\n\t\t\tWord: {}\n", data.guess_word);
    }

    println!();
    println!("\t1. Play Again");
    println!("\t2. To Menu");
    println!("\t3. Quit");
    print!("\tChoice: ");
    stdout().flush().unwrap();
    let choice: char = scan_char_lower();

    match choice {
        '1' => {
            *state = GameState::INGAME; 
            init_game_data(data);
        },
        '2' => *state = GameState::MENU,
        '3' => *state = GameState::QUIT,
        'q' => *state = GameState::QUIT,
        _ => {},
    }
}

fn display_man(lives: u8) {
    // l1
    if lives == 0 {
        println!("\t\t\t    ___");
    }
    // l2
    print!("\t\t\t   ");
    if lives <= 3 {
        print!("_");
    }
    else {
        print!(" ");
    }
    if lives <= 5 {
        print!("o");
    }
    if lives <= 3 {
        print!("_");
    }
    else {
        print!(" ");
    }
    if lives <= 1 {
        println!(" |");
    }
    else {
        println!();
    }
    // l3
    print!("\t\t\t   ");
    if lives <= 4 {
        print!(" | ");
    }
    if lives <= 1 {
        println!(" |");
    }
    else {
        println!();
    }
    // l4
    print!("\t\t\t   ");
    if lives <= 2 {
        print!("/ \\");
    }
    if lives <= 1 {
        println!(" |");
    }
    else {
        println!();
    }
    println!();
}

fn display_avail(avail_characters: &mut [bool; 26]) {
    let mut idx: usize = 0;
    print!("\t\t");
    while idx < 26 {
        if idx % 9 == 0 {
            println!();
            print!("\t\t");
        }

        if avail_characters[idx] {
            let ch: char = ('a' as u8 + idx as u8) as char;
            print!("{} ", ch);
        }
        else {
            print!("  ");
        }

        idx += 1;
    }
    println!("\n\n");
}

fn display_guess(progress: &mut String) {
    print!("\t\t\t");
    for ch in progress.as_bytes() {
        print!("{} ", *ch as char);
    }
    println!();
}

fn process_guess(data: &mut GameData, ch: char) {
    if ch < 'a' || ch > 'z' {
        return;
    }

    let char_idx = ch as usize - 'a' as usize;
    if !data.avail_characters[char_idx] {
        return;
    }
    data.avail_characters[char_idx] = false;

    let mut repl: Vec<usize> = Vec::new();
    
    let mut idx: usize = 0;
    let len = data.guess_word.len();
    while idx < len {
        if data.guess_word.as_bytes()[idx] == ch as u8 {
            repl.push(idx);
        }
        idx += 1;
    }

    if repl.len() == 0 as usize {
        data.lives -= 1;
        return;
    }

    for i in repl {
        data.progress.replace_range(i..i+1, ch.to_string().as_str());
    }
}

fn check_status(state: &mut GameState, data: &mut GameData) {
    if data.guess_word == data.progress {
        *state = GameState::END;
        data.win = true;
    }

    if data.lives == 0 {
        *state = GameState::END;
    }
}
