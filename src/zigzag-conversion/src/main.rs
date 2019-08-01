/*
* 题目地址 https://leetcode-cn.com/problems/zigzag-conversion/solution/zzi-xing-bian-huan-python-by-fei-ben-de-cai-zhu-uc/

## other
* 将 java 的实现翻译成 Rust 实现，结果是对的，但是还不十分理解
*/


fn main() {
    let str = String::from("LEETCODEISHIRING");
    let origin_str = str.clone();
    let res = Solution::convert(str, 3);
    println!("origin string is:{}", origin_str);
    println!("the result string is:{}", res);
}

#[derive(Debug)]
struct Solution ();

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let s_bytes = s.as_bytes();
        let mut rows:Vec<String> = vec![];
        let num_rows:usize = num_rows as usize;
        let mut flag1 = 0;
        let mut max_num = 0;
        loop {
            if s.len() > num_rows as usize {
                max_num = s.len();
            } else {
                max_num = num_rows as usize;
            }
            if flag1 > max_num {
                break;
            }
            rows.push(String::from(""));
            flag1 = flag1 + 1;
        }
        let mut cur_row = 0;
        let mut going_down:bool = false;
        for (_index, item) in s_bytes.iter().enumerate() {
            let tmp_char = *item as char;
            rows[cur_row].push(tmp_char);
            if cur_row == 0 || cur_row == num_rows as usize - 1 {
                going_down = !going_down
            }
            if going_down {
                cur_row += 1;
            } else {
                cur_row -= 1;
            }
        }
        let mut res_str = String::from("");
        for one_str in rows {
            res_str.push_str(&one_str);
        }
        res_str
    }
}
