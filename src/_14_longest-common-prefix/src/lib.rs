/// 题目地址：https://leetcode-cn.com/problems/longest-common-prefix/
/// 14. 最长公共前缀
/// 编写一个函数来查找字符串数组中的最长公共前缀。
/// 如果不存在公共前缀，返回空字符串 ""。

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let out = Solution::longest_common_prefix(input);
        assert_eq!("fl".to_string(), out);
    }
}
