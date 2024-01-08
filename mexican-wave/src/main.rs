use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let output: Vec<String> = wave(args.pop().expect("Expected String").to_lowercase());
    println!("{:?}", output);
}

// https://www.codewars.com/kata/58f5c63f1e26ecda7e000029
//
// In this simple Kata your task is to create a function that turns a string into a Mexican Wave.
// You will be passed a string and you must return that string in an array where an uppercase letter is a person standing up.
//
// 1. The input string will always be lower case but maybe empty.
// 2. If the character in the string is whitespace then pass over it as if it was an empty seat

// This is the naive solution from 05.01.2024
fn wave(input: String) -> Vec<String> {
    let mut wave: Vec<String> = Vec::new();
    let input_len: usize = input.len();

    if input_len == 0 {
        return wave;
    }

    for index in 1..input_len + 1 {
        let (head, tail) = input.split_at(index);

        let mut temp_input: String = "".to_string();
        if index == 1 {
            temp_input = head.to_uppercase() + tail;
        }
        if index > 1 {
            let (temp_head, upper_char) = head.split_at(index - 1);
            let new_char: String = if upper_char == "" {
                String::from(" ")
            } else {
                upper_char.to_uppercase()
            };
            temp_input = temp_head.to_owned() + new_char.as_str() + tail;
        }

        wave.push(temp_input)
    }

    return wave;
}

#[test]
fn it_has_empty_input_works() {
    let actual_result: Vec<String> = wave(String::from(""));
    let expected_result: Vec<String> = Vec::<String>::new();

    assert_eq!(actual_result, expected_result);
}

#[test]
fn it_has_single_character_works() {
    let actual_result: Vec<String> = wave(String::from("a"));
    let expected_result: Vec<String> = vec![String::from("A")];

    assert_eq!(actual_result, expected_result);
}

#[test]
fn it_has_single_word_works() {
    let actual_result: Vec<String> = wave(String::from("hello"));
    assert_eq!(
        actual_result,
        vec!["Hello", "hEllo", "heLlo", "helLo", "hellO"]
    );
}

#[test]
fn it_has_separated_words_by_space_works() {
    let actual_result: Vec<String> = wave(String::from("hello world"));
    assert_eq!(
        actual_result,
        vec![
            "Hello world",
            "hEllo world",
            "heLlo world",
            "helLo world",
            "hellO world",
            "hello world",
            "hello World",
            "hello wOrld",
            "hello woRld",
            "hello worLd",
            "hello worlD"
        ]
    );
}
