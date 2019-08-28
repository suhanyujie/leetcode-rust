/*
## 题
* 地址 https://leetcode-cn.com/problems/string-to-integer-atoi/

## 解题思路
* 有符号和无符号的处理
* 溢出处理

*/

fn main() {
    let str1 = String::from("-123你是谁");
    let str1_ori = str1.clone();
    let num1 = Solution::my_atoi(str1);
    println!("{} -> {}", str1_ori, num1);
}

#[derive(Debug)]
struct Solution ();

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let flag = b'-' as u8;
        let first = b'0' as u8;
        let latest = b'9' as u8;
        let str_bytes = str.as_bytes();
        let mut num_arr:Vec<&u8> = vec![];
        str_bytes.iter().all(|x| {
            if (*x != flag) &&
                (*x < first || *x > latest) {
                return false;
            }
            num_arr.push(x);
            true
        });
        num_arr.iter().map(|x| {
            if (x == '-') {
                return false;
            }
            
        });
        println!("{:?}", num_arr);
        0
    }
}
