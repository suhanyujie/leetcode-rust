//! 题目地址 https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/
//! 知识点：数组的合并

use std::collections::HashMap;
struct Solution;


impl Solution {
    /*
    # Rust 字符串的拼接

    */
    pub fn letter_combinations(tmp_str: String, digits: String) -> Vec<String> {
        let len1 = digits.len();
        if len1 == 0 {
            return vec![tmp_str];
        }
        let letter_map = Solution::get_letter_map();
        let bytes1 = digits.as_bytes();
        let mut res: Vec<String> = vec![];
        let arr1 = Solution::get_ref_arr(&bytes1[0]);
        for s1 in arr1 {
            let one_res = tmp_str.clone() + s1;
            res.push(one_res);
        }
        if digits.len() == 0 {
            return res;
        }
        let mut res2: Vec<String> = vec![];
        for one_res in res {
            let part_res = Solution::letter_combinations(one_res, digits[1..].to_string());
            // 数组的合并
            res2 = [res2, part_res].concat();
        }

        return res2;
    }

    fn get_ref_arr(c: &u8) -> Vec<&str> {
        let letter_map = Solution::get_letter_map();
        let arr1 = letter_map.get(&(*c as char));
        if arr1 == None {
            panic!("invalid number. ");
        }
        let arr1 = arr1.unwrap();
        return arr1.clone();
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
        assert_eq!(
            Solution::letter_combinations("".to_string(), "23".to_string()), 
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
