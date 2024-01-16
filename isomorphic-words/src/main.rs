fn main() {
    println!("Hello, world!");
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

// This is the naive solution from ??.??.2024
fn isomorphic_words(word_a: &str, word_b: &str) -> bool {
    todo!()
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
    fn test_is_false() {
        assert_eq!(isomorphic_words("AB", "CC"), false);
        assert_eq!(isomorphic_words("XXY", "XYY"), false);
        assert_eq!(isomorphic_words("ABAB", "CD"), false);
        assert_eq!(isomorphic_words("SEE", "SAW"), false);
    }
}
