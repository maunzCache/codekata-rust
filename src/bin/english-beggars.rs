fn main() {
    // TODO: Allow input from shell
    let output: Vec<u32> = english_beggars(&[1, 2, 3, 4, 5], 1);
    println!("{:?}", output);
}

// https://www.codewars.com/kata/59590976838112bfea0000fa
//
// Given an array of values and an amount of beggars
// you are supposed to return an array with the sum of what each beggar brings home,
// assuming they all take regular turns, from the first to the last.
//
// 1. Not all beggars have to take the same amount of "offers"
//    meaning that the length of the array is not necessarily a multiple of n;
//    length can be even shorter, in which case the last beggars will of course take nothing (0).
// 2. Do not modify the input array.

// This is the naive solution from 11.01.2024
fn english_beggars(values: &[u32], n: usize) -> Vec<u32> {
    let mut cashout: Vec<u32> = vec![];

    if n > 0 {
        for _index in 0..n {
            cashout.push(0)
        }

        let mut counter = 0;
        for index in 0..values.len() {
            cashout[counter] += values[index];
            counter = (counter + 1) % n;
        }
    }

    return cashout;
}

#[cfg(test)]
mod tests {
    use super::english_beggars;

    #[test]
    fn test_basic() {
        assert_eq!(english_beggars(&[1, 2, 3, 4, 5], 1), [15]);
        assert_eq!(english_beggars(&[1, 2, 3, 4, 5], 2), [9, 6]);
        assert_eq!(english_beggars(&[1, 2, 3, 4, 5], 3), [5, 7, 3]);
        assert_eq!(english_beggars(&[1, 2, 3, 4, 5], 6), [1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn test_zero_beggars() {
        assert_eq!(english_beggars(&[1, 2, 3, 4, 5], 0), []);
    }
}
