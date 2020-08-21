//! 题目地址：https://leetcode-cn.com/problems/integer-to-roman/
//! 贪心算法的规则如下：
//! >每一步都使用当前较大的罗马数字作为加法因子，最后得到罗马数字表示就是长度最少的。

struct Solution{}


impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let unit_list: Vec<(&'static str, i32)> = Solution::get_units();
        let mut cur_num = num;
        let mut num_str = String::new();
        loop {
            let suitable_index = Solution::get_suitable_unit_index(cur_num);
            let (unit_str, suitable_unit) = unit_list[suitable_index];
            cur_num = cur_num - 1 * suitable_unit;
            num_str += unit_str;
            if cur_num <=0 {
                break;
            }
        }

        num_str
    }

    fn get_suitable_unit_index(num: i32) -> usize {
        let unit_list: Vec<(&'static str, i32)> = Solution::get_units();
        let mut prev_index = 0;
        for i in 0..unit_list.len() {
            let (unit_str, unit) = unit_list[i];
            if num >= unit  {
                prev_index = i;
                break;
            }
        }
        prev_index
    }

    pub fn get_units() ->Vec<(&'static str, i32)> {
        let unit_list: Vec<(&'static str, i32)> = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        unit_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("II".to_string(), Solution::int_to_roman(2));
        assert_eq!("DCLXXI".to_string(), Solution::int_to_roman(671));
    }
}
