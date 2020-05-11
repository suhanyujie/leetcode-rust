/// https://leetcode-cn.com/problems/regular-expression-matching/
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
        if s.is_empty() || p.is_empty() {
            return false;
        }
        // 第一个字符匹配，则继续判断后续的字符是否匹配
        let c1 = s.chars().nth(0).unwrap();
        let c2 = p.chars().nth(0).unwrap();
        let len1 = s.len();
        let len2 = p.len();
        if c1 != c2 && c1 != '.' {
            return false;
            // return Solution::trace_back(s[1..], p[1..])
        }
        let s = &s[1..len1];
        let p = &p[1..len2];
        println!("{}", &s);
        println!("{}", &p);
        Solution::trace_back(s.to_string(), p.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_trace_back() {
        let s1 = String::from("this aaa");
        let p1 = String::from("this a*");
        let s1_1 = s1.clone();
        let res = Solution::trace_back(s1, p1);
        println!("the res is:{}", res);
        let l1 = s1_1.len();
        println!("the result is:{}", l1);
        assert_eq!(res, true);
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
