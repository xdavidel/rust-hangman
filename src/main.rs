extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io;
use std::io::prelude::*;

const ALLOWED_ATTEMPTS: u8 = 5;

#[derive(Debug)]
struct Letter {
    value: char,
    is_revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

impl Letter {
    fn from_string(word: &String) -> Vec<Letter> {
        let mut letters: Vec<Letter> = Vec::new();

        for c in word.chars() {
            letters.push(Letter {
                value: c,
                is_revealed: false,
            });
        }

        letters
    }
}

fn main() -> std::io::Result<()> {
    let mut attempts_left = ALLOWED_ATTEMPTS;

    let selected_word = select_word();

    let mut selected_letters = Letter::from_string(&selected_word);

    println!("Welcome to Hangman");

    loop {
        println!("You have {} turns left", attempts_left);

        display_progress(&selected_letters);

        println!("Please enter a letter to guess: ");
        let user_char = read_user_input();

        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed: bool = false;

        for letter in selected_letters.iter_mut() {
            if letter.value == user_char {
                letter.is_revealed = true;
                at_least_one_revealed = true;
            }
        }

        if !at_least_one_revealed {
            attempts_left -= 1;
        }

        match check_progress(attempts_left, &selected_letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                print!("\nCongrats, you have won!");
                break;
            }
            GameProgress::Lost => {
                print!("\nSorry, you have lost...");
                break;
            }
        }
    }

    println!("The word was '{}'\n\nGoodbye", selected_word);

    Ok(())
}

fn select_word() -> String {
    let mut file = File::open("words.txt").expect("Error: Could not open file");
    let mut words = String::new();
    file.read_to_string(&mut words)
        .expect("Error: Could not read file");

    let available_words: Vec<&str> = words.trim().split(',').collect();

    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");
    for letter in letters {
        display_string.push(' ');

        if letter.is_revealed {
            display_string.push(letter.value);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => {
                return c;
            }
            None => {
                return '*';
            }
        },
        Err(_) => {
            return '*';
        }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    if turns_left <= 0 {
        return GameProgress::Lost;
    }

    let mut all_revealed = true;

    for letter in letters {
        if !letter.is_revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        GameProgress::Won
    } else {
        GameProgress::InProgress
    }
}
