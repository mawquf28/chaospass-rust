use arboard::Clipboard;
use clap::{Parser, ValueEnum};
use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;
use zeroize::{Zeroize, Zeroizing};

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Chaos,
    Diceware,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Cryptographically secure CLI password generator", long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Mode::Chaos)]
    mode: Mode,

    #[arg(short, long, default_value_t = 16)]
    length: usize,

    #[arg(short, long, default_value_t = 5)]
    words: usize,
}

const EFF_WORDLIST: &str = include_str!("eff_large_wordlist.txt");

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

fn main() {
    let args = Args::parse();
    let password = Zeroizing::new(match args.mode {
        Mode::Chaos => generate_chaos(args.length),
        Mode::Diceware => generate_diceware(args.words),
    });

    println!("Generated password: {}", *password);
    if let Ok(mut clipboard) = Clipboard::new() {
        if clipboard.set_text((*password).clone()).is_ok() {
            println!("Password copied to clipboard. The program will wait 15 seconds to clear it...");

            let mut secret_to_clear = Zeroizing::new((*password).clone());
            thread::sleep(Duration::from_secs(15));
            
            if let Ok(mut cb) = Clipboard::new() {
                if let Ok(current_text) = cb.get_text() {
                    if current_text == *secret_to_clear {
                        let _ = cb.set_text("");
                        println!("Clipboard successfully cleared.");
                    } else {
                        println!("Clipboard content changed by user, skip clearing.");
                    }
                }
            }
            secret_to_clear.zeroize();
        }
    }
}

fn generate_chaos(length: usize) -> String {
    if length < 4 {
        panic!("Length must be at least 4 to cover all character classes");
    }

    let mut rng = rand::rngs::OsRng;
    let all_chars = [LOWERCASE, UPPERCASE, DIGITS, SYMBOLS].concat();
    let mut bytes = vec![
        *LOWERCASE.choose(&mut rng).unwrap(),
        *UPPERCASE.choose(&mut rng).unwrap(),
        *DIGITS.choose(&mut rng).unwrap(),
        *SYMBOLS.choose(&mut rng).unwrap(),
    ];

    for _ in 4..length {
        bytes.push(*all_chars.choose(&mut rng).unwrap());
    }

    bytes.shuffle(&mut rng);

    String::from_utf8(bytes).expect("Failed to create valid UTF-8 string")
}

fn generate_diceware(num_words: usize) -> String {
    let mut rng = rand::rngs::OsRng;
    let words: Vec<&str> = EFF_WORDLIST
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .collect();

    let mut selected_words = Vec::with_capacity(num_words);
    for _ in 0..num_words {
        let word = words.choose(&mut rng).unwrap();
        selected_words.push(*word);
    }

    selected_words.join("-")
}