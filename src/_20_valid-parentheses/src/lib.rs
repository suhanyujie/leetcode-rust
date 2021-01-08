//! 题目名称：20. 有效的括号
//! 题目地址：https://leetcode-cn.com/problems/valid-parentheses/

use std::collections::HashMap;
/*
给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。

有效字符串需满足：

左括号必须用相同类型的右括号闭合。
左括号必须以正确的顺序闭合。
注意空字符串可被认为是有效字符串。

示例 1:
输入: "()"
输出: true

示例 2:
输入: "()[]{}"
输出: true

输入: "()[]{[()()[]]}"
输出: true
*/
struct Solution;

impl Solution {
    /// 左右括号分类存放在不同的变量中，以便判断。
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let str_map = Self::get_str_pair();
        let res = s.chars().find_map(|c| {
            if Self::is_left(c) {
                stack.push(c);
            } else {
                let pop_val = stack.pop();
                if pop_val.is_none() {
                    return Some(true);
                }
                let matched_char = str_map.get(&pop_val.unwrap());
                //println!("cur char is: {}, matched char is: {:?}, poped val: {:?}", c, matched_char, pop_val);
                if matched_char.is_none() {
                    return Some(true);
                }
                if matched_char.unwrap().ne(&c) {
                    return Some(true);
                }
            }
            return None;
        });
        if res.is_none() && stack.len() == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_str_pair() ->HashMap<char, char> {
        let mut m: HashMap<char, char> = HashMap::new();
        m.insert('(', ')');
        m.insert('[', ']');
        m.insert('{', '}');
        return m;
    }

    pub fn is_left(c: char) -> bool {
        return vec!['(', '{', '['].contains(&c);
    }

    pub fn is_right(c: char) -> bool {
        return vec![')', '}', ']'].contains(&c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let is_ok = Solution::is_valid("()[]{}".to_string());
        assert!(is_ok);
        assert!(!Solution::is_valid("([]{}".to_string()));
    }
}
