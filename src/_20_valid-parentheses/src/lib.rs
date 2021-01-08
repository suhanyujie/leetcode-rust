//! 题目名称：20. 有效的括号
//! 题目地址：https://leetcode-cn.com/problems/valid-parentheses/
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
        s.chars().map(|c| Self::is_left(c))
    }

    pub fn is_left(c: char) ->bool {
        return vec!['(', '{', '['].contains(&c);
    }

    pub fn is_right(c: char) ->bool {
        return vec![')', '}', ']'].contains(&c);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
