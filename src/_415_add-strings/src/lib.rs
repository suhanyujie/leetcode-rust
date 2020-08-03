//! 领扣地址：https://leetcode-cn.com/problems/add-strings/
//! ## 给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和。
//! * num1 和num2 的长度都小于 5100.
//! * num1 和num2 都只包含数字 0-9
//! * num1 和num2 都不包含任何前导零。
//! * 你不能使用任何內建 BigInteger 库， 也不能直接将输入的字符串转换为整数形式。

struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let b1 = num1.as_bytes();
        let b2 = num2.as_bytes();
        let mut i = 0;
        let mut j = 0;
        let mut add = 0;
        let mut res = String::from("");
        loop {
            let n1 = b1[i] - '0' as u8;
            let n2 = b2[i]- '0' as u8;
            if n1 + n2 > 9 {
                add = 1;
                let single_res = (n1 + n2) %10;
                res.push(single_res as char)
            }
            println!("{}-{}=>{}", n1, n2, res);
            break;
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Solution::add_strings("923".to_string(), "328".to_string());
        assert!(false);
    }

    #[test]
    fn test_my_test() {
        assert_eq!(1, '1' as u8 - '0' as u8);
        assert_eq!('2', 2 as char);
    }
}
