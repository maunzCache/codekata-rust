// use std::env;

fn main() {
    // let mut args: Vec<String> = env::args().collect(); // TODO: Allow to use input from shell. Might force another data type as input.
    let output: char = find_missing_letter(&['a', 'c', 'd', 'e']);
    println!("{:?}", output);
}

// https://www.codewars.com/kata/5839edaa6754d6fec10000a2
//
// Write a method that takes an array of consecutive (increasing) letters as input and that returns the missing letter in the array.
//
// 1. You will always get an valid array.
// 2. And it will be always exactly one letter be missing.
// 3. The length of the array will always be at least 2.
// 4. The array will always contain letters in only one case (upper or lower).
// 5. The input will only contain letters from the english alphabet

// This is the naive solution from 05.01.2024
fn find_missing_letter(chars: &[char]) -> char {
    let mut missing_letter = '\0' as u32;

    let mut current_letter = '\0' as u32;
    for index in 0..chars.len() {
        if current_letter == '\0' as u32 {
            current_letter = chars[index] as u32;
            continue;
        }
        let previous_letter = current_letter as u32;
        current_letter = chars[index] as u32;

        if current_letter - previous_letter > 1 {
            missing_letter = current_letter - 1 as u32;
        }
        if missing_letter != '\0' as u32 {
            break;
        }
    }

    return char::from_u32(missing_letter).expect("Kaputt");
}

#[test]
fn it_has_lower_case_letters_works() {
    assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
}

#[test]
fn it_has_upper_case_letters_works() {
    assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
}

// TODO: Create smoke test which randomly tests every letter in the alphabet :D
