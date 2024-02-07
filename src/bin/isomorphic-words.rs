use std::collections::HashMap;

fn main() {
    // TODO: Allow input from shell
    let output: bool = isomorphic_words("CBAABC", "DEFFED");
    println!("{:?}", output);
}

// https://www.codewars.com/kata/59dbab4d7997cb350000007f
//
// Two strings a and b are called isomorphic if
// there is a one to one mapping possible for every character of a to every character of b.
// And all occurrences of every character in a map to same character in b.
//
// 1. Your solution must be able to handle words with more than 10 characters.
// 2. Remember that order is important.
// 3. Return true if two given strings are isomorphic to each other, and false otherwise.

// This is the naive solution from 18.01.2024
fn isomorphic_words(word_a: &str, word_b: &str) -> bool {
    if word_a.len() != word_b.len() {
        return false;
    }
    let word_len: usize = word_a.len();

    // Create character mapping for word_a
    let mut word_a_map: HashMap<&str, &str> = HashMap::new();
    let mut word_b_map: HashMap<&str, &str> = HashMap::new();

    for index in 0..word_len {
        let char_from_a: &str = word_a.get(index..index + 1).unwrap();
        let char_from_b: &str = word_b.get(index..index + 1).unwrap();

        // If character is not in map: create new entry
        if word_a_map.contains_key(char_from_a) {
            // If character is in map: value (mapping) must be equal to what is provided
            if word_a_map.get(char_from_a).unwrap() != &char_from_b {
                // Else the words cannot be isomorphic
                return false;
            }
        } else {
            word_a_map.insert(char_from_a, char_from_b);
        }

        // Now we must check if the value was already provided too
        // Same code routine as before but map switched
        if word_b_map.contains_key(char_from_b) {
            if word_b_map.get(char_from_b).unwrap() != &char_from_a {
                return false;
            }
        } else {
            word_b_map.insert(char_from_b, char_from_a);
        }
    }

    // Additional assertion just to be super sure not to mess anything up
    // Totally uneccessary
    if word_a_map.len() != word_b_map.len() {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::isomorphic_words;

    #[test]
    fn test_is_true() {
        assert_eq!(isomorphic_words("CBAABC", "DEFFED"), true);
        assert_eq!(isomorphic_words("XXX", "YYY"), true);
        assert_eq!(isomorphic_words("RAMBUNCTIOUSLY", "THERMODYNAMICS"), true);
        assert_eq!(isomorphic_words("ESTATE", "DUELED"), true);
    }

    #[test]
    fn test_its_value_maps_twice_is_false() {
        assert_eq!(isomorphic_words("AB", "CC"), false);
    }

    #[test]
    fn test_its_argument_word_length_not_equal_is_false() {
        assert_eq!(isomorphic_words("ABAB", "CD"), false);
    }

    #[test]
    fn test_is_false() {
        assert_eq!(isomorphic_words("XXY", "XYY"), false);
        assert_eq!(isomorphic_words("SEE", "SAW"), false);
    }
}
