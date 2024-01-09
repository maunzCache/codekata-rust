fn main() {
    println!("Hello, world!");
}

// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust
//
// You probably know the "like" system from Facebook and other pages.
// People can "like" blog posts, pictures or other items.
// We want to create the text that should be displayed next to such an item.
//
// Implement the function which takes an array containing the names of people that like an item.
// []                                -->  "no one likes this"
// ["Peter"]                         -->  "Peter likes this"
// ["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
// ["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
// ["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"
// Note: For 4 or more names, the number in "and 2 others" simply increases.

// This is the naive solution from 08.01.2024
fn likes(names: &[&str]) -> String {
    let entries = names.len();

    match entries {
        1 => return String::from(names[0].to_owned() + " likes this"),
        2 => return String::from(names[0].to_owned() + " and " + names[1] + " like this"),
        3 => {
            return String::from(
                names[0].to_owned() + ", " + names[1] + " and " + names[2] + " like this",
            )
        }
        4..=usize::MAX => {
            return String::from(
                names[0].to_owned()
                    + ", "
                    + names[1]
                    + " and "
                    + &(entries - 2).to_string().as_str()
                    + " others like this",
            )
        }
        _ => return String::from("no one likes this"),
    }
}

// TODO: Add smoke test for all sizes within usize:MAX
// TODO: Refactor these tests to look like the other katas
#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
        assert_eq!(
            likes(&["Bobby", "Alex", "Jacob", "Mark", "Max"]),
            "Bobby, Alex and 3 others like this"
        );
    }
}
