/// https://leetcode-cn.com/problems/regular-expression-matching/

/*
## 题目描述
给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。

'.' 匹配任意单个字符
'*' 匹配零个或多个前面的那一个元素

## 示例
输入:
s = "aa"
p = "a"
输出: false
解释: "a" 无法匹配 "aa" 整个字符串。

输入:
s = "aa"
p = "a*"
输出: true
解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。

输入:
s = "ab"
p = ".*"
输出: true
解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。

输入:
s = "aab"
p = "c*a*b"
输出: true
解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/regular-expression-matching
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

///
/// 回溯法
///
#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        return Solution::trace_back(s, p);
    }
    /// 回溯法
    fn trace_back(s: String, p: String) -> bool {
        if p.is_empty() {
            return true;
        }
        if s.is_empty() && p.is_empty() {
            return true;
        }
        let s_vec: Vec<char> = s.chars().collect();
        let p_vec: Vec<char> = p.chars().collect();
        for c1 in s_vec {
            
        }
        println!("{:?}", s_vec);
        return false;
        // Solution::trace_back(s.to_string(), p.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_trace_back() {
        assert_eq!(Solution::is_match("this aaa".to_string(), "this a*".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
    }
}
