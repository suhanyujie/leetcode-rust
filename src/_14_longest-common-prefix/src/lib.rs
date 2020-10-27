/// 题目地址：https://leetcode-cn.com/problems/longest-common-prefix/
/// 14. 最长公共前缀
/// 编写一个函数来查找字符串数组中的最长公共前缀。
/// 如果不存在公共前缀，返回空字符串 ""。

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut returnString = String::new();
        if strs.len() < 1 {
            return returnString;
        }
        let sample_str = strs.get(0).unwrap();
        let mut sample_str_bytes = sample_str.as_bytes();
        for index in 1.. strs.len() {
            let mut cmp_index = 0;
            while let Some(cmp_str) = strs.get(index) {
                let cmp_str_bytes = cmp_str.as_bytes();
                if cmp_index >= sample_str_bytes.len() || 
                    cmp_index >=  cmp_str_bytes.len() || 
                    sample_str_bytes[cmp_index] != cmp_str_bytes[cmp_index] {
                    break;
                }
                cmp_index += 1;
            }
            sample_str_bytes = &sample_str_bytes[0..cmp_index];
            if sample_str_bytes.len() <= 0 {
                break;
            }
        }
        String::from_utf8_lossy(sample_str_bytes).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let out = Solution::longest_common_prefix(input);
        assert_eq!("fl".to_string(), out);
    }
}
