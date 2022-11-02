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

## 参考的一些题解
- https://leetcode.cn/problems/regular-expression-matching/solutions/1418712/by-lebron-s-u27g/

*/

///
/// 回溯法
///
#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s1 = format!("{}{}", " ", s);
        let p1 = format!("{}{}", " ", p);
        let s_vec: Vec<char> = s1.chars().collect();
        let p_vec: Vec<char> = p1.chars().collect();
        return Solution::style_1(s_vec, p_vec);
    }

    /// 从右往前匹配  类似于动态规划
    fn style_1(s: Vec<char>, p: Vec<char>) -> bool {
        if p.is_empty() {
            return true;
        }
        if s.is_empty() || p.is_empty() {
            return true;
        }
        let m = s.len();
        let n = p.len();
        // init dp status array
        let mut dp: Vec<Vec<bool>> = Vec::with_capacity(m);
        for _ in 0..=n {
            let mut tmp_arr2: Vec<bool> = vec![];
            for _ in 0..=m {
                tmp_arr2.push(false);
            }
            dp.push(tmp_arr2);
        }

        for i in 1..n {
            if p[i] == '*' {
                if i - 2 >= 0 {
                    dp[i][0] = dp[i - 2][0];
                }
            }
        }
        // 这种情况一定匹配
        dp[0][0] = true;
        for i in 1..n {
            for j in 1..m {
                if p[i] == s[j] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[i] == '.' {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p[i] == '*' {
                    // 此情况最复杂
                    if p[i - 1] != s[j] && p[i - 1] != '.' {
                        dp[i][j] = dp[i - 2][j];
                    } else {
                        dp[i][j] = dp[i - 2][j - 1] || dp[i][j - 1];
                    }
                }
            }
        }

        return dp[n - 1][m - 1];
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_trace_back() {
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
        assert_eq!(
            Solution::is_match("this aaa".to_string(), "this a*".to_string()),
            true
        );
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
    }

    #[test]
    fn test_some_func() {
        let s1 = r#"this is"#.to_string();
        let arr1: Vec<char> = s1.chars().collect();
        let i = arr1.len() - 1;
        println!("{:?}", arr1[0..=i].to_vec());
        assert!(false);
    }

    #[test]
    fn test_vec_len1() {
        let arr1 = [0; 10];
        dbg!(arr1);
    }
}
