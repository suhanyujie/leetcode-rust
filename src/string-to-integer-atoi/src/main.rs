/*
## 题
* 地址 https://leetcode-cn.com/problems/string-to-integer-atoi/

## 解题思路
* 有符号和无符号的处理
* 溢出处理

*/

fn main() {
    let str1 = String::from("  -123你是21谁");
    let str1_ori = str1;
    let num1 = Solution::my_atoi(str1_ori.to_string());
    println!("{} -> {}", str1_ori, num1);
}

#[derive(Debug)]
struct Solution();

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let str_bytes = str.trim().as_bytes();
        let mut num_arr: Vec<u8> = vec![];
        let mut flag = 1;
        str_bytes.iter().enumerate().all(|(index, x)| {
            if index == 0 {
                if *x == b'-' {
                    flag = -1;
                    return true;
                } else if *x == b'+' {
                    flag = 1;
                    return true;
                }
            }
            if *x == b'-' && index != 0 {
                return false;
            }
            if (*x != b'-') && !Self::is_digit(*x) {
                return false;
            }
            num_arr.push(*x);
            true
        });
        let mut res_int: i32 = 0;
        let mut has_overflow = false;
        num_arr.iter().enumerate().for_each(|(i, x)| {
            if i == 0 && *x == b'-' {
                flag = -1;
            } else {
                if let Ok(tmp_int) = (*x as char).to_string().parse::<u8>() {
                    if let Some(mul_num) = i32::checked_mul(res_int, 10) {
                        if let Some(num) = i32::checked_add(mul_num, tmp_int as i32) {
                            res_int = num;
                        } else {
                            has_overflow = true;
                        }
                    } else {
                        has_overflow = true;
                    }
                }
            }
        });
        res_int *= flag;
        if has_overflow {
            if flag == -1 {
                res_int = i32::MIN;
            } else {
                res_int = i32::MAX;
            }
        }
        res_int
    }

    fn is_digit(x: u8) -> bool {
        x >= b'0' && x <= b'9'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_itoa1() {
        println!("max i32: {}, min: {}", i32::MAX, i32::MIN);
        // 2147483647
        // 91283472332
        // -912834723
    }

    #[test]
    fn test_itoa2() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("+2".to_string()), 2);
    }
}
