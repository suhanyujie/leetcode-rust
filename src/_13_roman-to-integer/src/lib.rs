//! 题目地址：https://leetcode-cn.com/problems/roman-to-integer/
//! tips：这题和[第 12 题](https://leetcode-cn.com/problems/integer-to-roman)很有关联性，它们互为逆过程。
//! 优化方向：预先设定的特殊罗马数值表，使用 hashmap，这样在查询的时候，提升性能。

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res_num = 0;
        let char_arr: Vec<char> = s.chars().rev().collect();
        let mut first_char: char;
        let mut second_char: char;
        let arr_len = char_arr.len();
        let mut tmp_unit: String = String::new();
        // 查看两个字符
        let mut i = 0;
        while i < arr_len {
            tmp_unit = "".to_string();
            first_char = char_arr[i];
            // “前看”一个字符，如果字符存在，则尝试一次查询两个字符组成的罗马数字是否存在
            // 不存在则用一个字符查询，此时一定存在
            // 存在时，则跳过前看的这个字符
            if i + 1 < arr_len {
                second_char = char_arr[i + 1];
                tmp_unit.push(second_char);
                tmp_unit.push(first_char);
                if let Some(num1) = Solution::chech_unit_exist(&tmp_unit) {
                    res_num += num1;
                    i += 2;
                } else {
                    tmp_unit = "".to_string();
                    // 前看后查询不到，则使用单个字符查询
                    tmp_unit.push(first_char);
                    if let Some(num1) = Solution::chech_unit_exist(&tmp_unit) {
                        res_num += num1;
                    }
                    i += 1;
                }
            } else {
                // 如果前看的字符不存在，则只能用单个字符去查询
                tmp_unit.push(first_char);
                if let Some(num1) = Solution::chech_unit_exist(&tmp_unit) {
                    res_num += num1;
                }
                i += 1;
            }
        }
        res_num
    }

    // 检查数值是否是特殊的数值
    pub fn chech_unit_exist(s1: &str) -> Option<i32> {
        let unit_list: HashMap<&'static str, i32> = Solution::get_units();
        if let Some(num) = unit_list.get(s1) {
            Some(*num)
        } else {
            None
        }
    }

    pub fn get_units() -> HashMap<&'static str, i32> {
        let mut unit_list = HashMap::new();
        unit_list.insert("M", 1000);
        unit_list.insert("CM", 900);
        unit_list.insert("D", 500);
        unit_list.insert("CD", 400);
        unit_list.insert("C", 100);
        unit_list.insert("XC", 90);
        unit_list.insert("L", 50);
        unit_list.insert("XL", 40);
        unit_list.insert("X", 10);
        unit_list.insert("IX", 9);
        unit_list.insert("V", 5);
        unit_list.insert("IV", 4);
        unit_list.insert("I", 1);
        
        unit_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(4, Solution::roman_to_int("IV".to_string()));
        assert_eq!(5, Solution::roman_to_int("V".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
