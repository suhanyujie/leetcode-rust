/*
题目地址 https://leetcode-cn.com/problems/reverse-integer/

*/
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let res = Solution::reverse(9102);
    println!("{}", res);
    let res = Solution::reverse(1534236469);
    println!("{}", res);
    let res = Solution::reverse(-123);
    println!("{}", res);
}

#[derive(Debug)]
struct Solution ();

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x>0 && x < 10 {
            return x;
        }
        let mut x:i32 = x;
        let mut flag = 1;
        if x < 0 {
            flag = -1;
            x = x * flag;
        }
        let str_bytes = x.to_string();
        let mut str_bytes:Vec<u8> = str_bytes.as_bytes().to_vec();
        str_bytes.reverse();
        let mut str_res = String::from("");
        let _:Vec<char> = str_bytes.iter().map(|x|{
            let c1 = *x as char;
            str_res.push(c1);
            c1
        }).collect();
        let str_res = i32::from_str(&str_res);
        let str_res = match str_res {
            Ok(res) => {
                res
            },
            Err(err) => {
                // println!("{:?}", err);
                0
            },
        };
        (flag * str_res)
    }
}
