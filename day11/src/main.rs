use std::collections::HashSet;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    // We represent a password as an array of base 23 (26 letters except i, o, and l)
    let mut current_password = string_to_base23(&args.input);

    while !valid_password(&current_password) {
        increment_password(&mut current_password);
        println!("{}", base23_to_string(&current_password));
    }

    println!("Next password: {}", base23_to_string(&current_password));
}

fn base23_to_string(input: &Vec<u8>) -> String {
    input.iter().fold("".to_string(), |acc, value| {
        acc + &base23_to_char(*value).to_string()
    })
}

fn base23_to_char(input: u8) -> char {
    let mut input = input + ('a' as u8);
    if input >= 'i' as u8 {
        input += 1;
    }
    if input >= 'l' as u8 {
        input += 1;
    }
    if input >= 'o' as u8 {
        input += 1;
    }

    input as char
}

fn string_to_base23(input: &str) -> Vec<u8> {
    input.chars().map(|c| char_to_base23(c)).collect()
}

fn char_to_base23(input: char) -> u8 {
    let mut number = input as u8;
    // Ordering is important here!
    if number > 'o' as u8 {
        number -= 1;
    }
    if number > 'l' as u8 {
        number -= 1;
    }
    if number > 'i' as u8 {
        number -= 1;
    }

    number - ('a' as u8)
}

fn increment_password(current_password: &mut Vec<u8>) {
    // FIXME: This can be a lot faster by skipping entire series where you can't have two duplicates
    for i in (0..current_password.len()).rev() {
        current_password[i] += 1;
        if current_password[i] >= 23 {
            current_password[i] = 0;
        } else {
            return;
        }
    }
}

fn valid_password(password: &Vec<u8>) -> bool {
    if get_unique_duplicate_count(&password) < 2 {
        return false;
    }
    if !has_straight(&password) {
        return false;
    }

    true
}

fn get_unique_duplicate_count(password: &Vec<u8>) -> usize {
    let duplicates = password
        .iter()
        .fold((100, HashSet::new()), |state, c| {
            let mut state = state.clone();
            if state.0 == *c {
                state.1.insert(*c);
            }

            state.0 = *c;
            state
        })
        .1;

    duplicates.len()
}

fn has_straight(password: &Vec<u8>) -> bool {
    let bad_points = vec![
        ('i' as u8 - 'a' as u8),
        ('i' as u8 - 'a' as u8) + 1,
        ('i' as u8 - 'a' as u8) + 2,
        ('l' as u8 - 'a' as u8),
        ('l' as u8 - 'a' as u8) + 1,
        ('l' as u8 - 'a' as u8) + 2,
        ('o' as u8 - 'a' as u8),
        ('o' as u8 - 'a' as u8) + 1,
        ('o' as u8 - 'a' as u8) + 2,
    ];
    for i in 2..password.len() {
        if password[i - 2] + 2 == password[i - 1] + 1
            && password[i - 1] + 1 == password[i]
            && !bad_points.contains(&password[i])
        {
            return true;
        }
    }

    false
}
