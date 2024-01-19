fn main() {
    println!("Hello, world!");
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

fn reverse_inside_parentheses(input: &str) -> &str {
    todo!()
}

#[cfg(test)]
mod test {
    use super::reverse_inside_parentheses;

    #[test]
    fn all_tests() {
        assert_eq!(reverse_inside_parentheses("h(el)lo"), "h(le)lo");
        assert_eq!(reverse_inside_parentheses("a ((d e) c b)"), "a (b c (d e))");
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
