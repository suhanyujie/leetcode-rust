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
        let mut i = (b1.len() - 1) as isize;
        let mut j = (b2.len() - 1) as isize;
        let mut add = 0;
        let mut res = String::from("");
        let mut n1: u8 = 0;
        let mut n2: u8 = 0;
        loop {
            n1 = if i < 0 { 0 } else { b1[i as usize] - '0' as u8 };
            n2 = if j < 0 { 0 } else { b2[j as usize] - '0' as u8 };
            let single_bit_res = n1 + n2 + add;
            let mut tmp_ch: u8 = 0;
            if single_bit_res > 9 {
                add = 1;
                let single_res = single_bit_res % 10;
                tmp_ch = single_res;
            } else {
                add = 0;
                tmp_ch = single_bit_res;
            }
            res = tmp_ch.to_string() + res.as_ref();
            i -= 1;
            j -= 1;
            if i<0 && j<0 {
                if add > 0 {
                    res = '1'.to_string() + res.as_ref();
                }
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "1251".to_string(),
            Solution::add_strings("923".to_string(), "328".to_string())
        );
    }

    #[test]
    fn test_my_test() {
        // 字符转换为数值类型
        assert_eq!(1, '1' as u8 - '0' as u8);
        // 数值类型转换为字符类型
        assert_eq!('2' as u8, ('0' as u8) + 2);
        // Rust 字符串和字符类型的拼接
        let tmp_s1 = String::from(" hello");
        assert_eq!(
            '1'.to_string() + tmp_s1.as_ref(),
            "1 hello"
        );
    }
}
