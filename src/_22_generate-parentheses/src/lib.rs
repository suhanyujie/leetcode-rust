use std::vec;

/// 名称：22. 括号生成
/// 来源：https://leetcode-cn.com/problems/generate-parentheses/

/*
数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

示例：

输入：n = 3
输出：[
       "((()))",
       "(()())",
       "(())()",
       "()(())",
       "()()()"
     ]
*/
struct Solution;

impl Solution {
    /// 排列组合，当求 n  对括号的所有可能性时，可视为 n-1 对括号的可能性 + 2（在第 n 个括号中，还是在第 n 个括号的右侧这 2 种可能性）
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        // 占位，方式所有结果
        let mut result: Vec<String> = vec![];
        if n == 0 {
            return result;
        } else if n == 1 {
            result.push("()".to_string());
        } else {
            let left: isize = n as isize;
            let right: isize = n as isize;
            Self::dfs("".to_string(), left, right, &mut result);
        }

        return result;
    }

    pub fn dfs(one_res: String, left: isize, right: isize, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            res.push(one_res);
            return;
        }
        if left > right {
            return;
        }
        if left > 0 {
            let s = (&*one_res).to_string() + "(";
            Self::dfs(s, left - 1, right, res);
        }
        if right > 0 {
            let s = (&*one_res).to_string() + ")";
            Self::dfs(s, left, right - 1, res);
        }
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let res = Solution::generate_parenthesis(3);
        assert_eq!(res, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }
}
