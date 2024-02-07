fn main() {
    // TODO: Allow input from shell
    let output: String = reverse_inside_parentheses("h(el)lo");
    println!("{:?}", output);
}

// https://www.codewars.com/kata/5e07b5c55654a900230f0229
//
// You will be given a string of text and valid parentheses, such as "h(el)lo".
// You must return the string, with only the text inside parentheses reversed, so "h(el)lo" becomes "h(le)lo".
// When parentheses are reversed, they should switch directions, so they remain syntactically correct (i.e. "h((el)l)o" becomes "h(l(el))o")
// However, if said parenthesized text contains parenthesized text itself, then that too must reversed back, so it faces the original direction.
//
// 1. There may be multiple groups of parentheses at any level (i.e. "(1) (2 (3) (4))").
// 2. Input parentheses will always be valid (i.e. you will never get "(()").

// This is the naive solution from ??.??.2024
fn reverse_inside_parentheses(input: &str) -> String {
    // Find first and last parentheses
    // Split word after opening parentheses and remember index to insert the reverse later
    // Repeat in substring until all parentheses were found
    // Reverse words and insert back into previous substring until the string is whole again
    let opening_parentheses_index = input.find('(').unwrap() + 1;
    let closing_parentheses_index = input.rfind(')').unwrap();
    let substring = input
        .get(opening_parentheses_index..closing_parentheses_index)
        .unwrap();
    let reversed_substring: String = substring.chars().rev().collect();

    let (old_head, _) = input.split_at(opening_parentheses_index);
    let (_, old_tail) = input.split_at(closing_parentheses_index);

    let new_string = old_head.to_owned() + reversed_substring.as_str() + old_tail;

    return new_string;
}

#[cfg(test)]
mod test {
    use super::reverse_inside_parentheses;

    #[test]
    fn it_contains_one_parentheses_is_reversed() {
        assert_eq!(reverse_inside_parentheses("h(el)lo"), "h(le)lo");
    }

    #[test]
    fn it_contains_more_parentheses_is_reversed() {
        assert_eq!(reverse_inside_parentheses("a ((d e) c b)"), "a (b c (d e))");
    }

    #[test]
    fn it_reverses_reversed_strings() {
        assert_eq!(
            reverse_inside_parentheses("one (two (three) four)"),
            "one (ruof (three) owt)"
        );
        assert_eq!(
            reverse_inside_parentheses("one (ruof ((rht)ee) owt)"),
            "one (two ((thr)ee) four)"
        );
    }
}
