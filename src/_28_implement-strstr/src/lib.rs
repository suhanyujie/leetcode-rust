/// 题目地址 https://leetcode-cn.com/problems/implement-strstr/

struct Solution;

/*
实现 strStr() 函数。

给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从0开始)。如果不存在，则返回  -1。

输入: haystack = "hello", needle = "ll"
输出: 2
输入: haystack = "aaaaa", needle = "bba"
输出: -1

解释: 当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。

对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与C语言的 strstr() 以及 Java的 indexOf() 定义相符。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/implement-strstr

// tips：看完题解，跟自己一开始的解法不一样。
题解之一的解法是基于滑动窗口。

*/

///
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let bytes1 = haystack.as_bytes();
        let needle_bytes = needle.as_bytes();
        let mut return_flag = -1;
        if bytes1.len() < needle_bytes.len() {
            return return_flag;
        }
        if needle_bytes.len() < 1 {
            return_flag = 0;
            return return_flag;
        }
        if bytes1.len() < 1 {
            return return_flag as i32;
        }
        for index1 in 0..bytes1.len() {
            for index2 in 0..needle_bytes.len() {
                if index1 + index2 >= bytes1.len() {
                    break;
                }
                if needle_bytes[index2] != bytes1[index1 + index2] {
                    break;
                } else {
                    if index2 == needle_bytes.len() - 1 {
                        return_flag = index1 as i32;
                    }
                }
            }
            if return_flag != -1 {
                break;
            }
        }
        return_flag as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, Solution::str_str("".to_string(), "".to_string()));
        assert_eq!(
            0,
            Solution::str_str("world".to_string(), "world".to_string())
        );
        assert_eq!(
            -1,
            Solution::str_str("mississippi".to_string(), "issipi".to_string())
        );
        assert_eq!(-1, Solution::str_str("aaa".to_string(), "aaaa".to_string()));
        assert_eq!(1, Solution::str_str("nishi".to_string(), "i".to_string()));
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
        assert_eq!(
            -1,
            Solution::str_str("aaaaa".to_string(), "bba".to_string())
        );
    }
}
