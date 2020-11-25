//! 题目地址 https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/
//! 

use std::collections::HashMap;
struct Solution;


impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let len1 = digits.len();
        let letter_map = Solution::get_letter_map();
        // digits.chars().enumerate().fold("", |acc, (_index, letter)| {

        // });
        vec!["".to_owned()]
    }

    fn get_letter_map() -> HashMap<char, Vec<&'static str>> {
        let mut letter_map: HashMap<char, Vec<&str>> = HashMap::new();
        letter_map.insert('2', vec!["a", "b", "c"]);
        letter_map.insert('3', vec!["d", "e", "f"]);
        letter_map.insert('4', vec!["g", "h", "i"]);
        letter_map.insert('5', vec!["j", "k", "l"]);
        letter_map.insert('6', vec!["m", "n", "o"]);
        letter_map.insert('7', vec!["p", "q", "r", "s"]);
        letter_map.insert('8', vec!["t", "u", "v"]);
        letter_map.insert('9', vec!["w", "x", "y", "z"]);
        return letter_map;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
