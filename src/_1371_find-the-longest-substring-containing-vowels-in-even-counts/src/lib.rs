/// 标题：1371. 每个元音包含偶数次的最长子字符串
/// 地址：https://leetcode-cn.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/

struct Solution;

impl Solution {
    /// 
    pub fn find_the_longest_substring(s: String) -> i32 {
        let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        s.as_bytes().iter().for_each(|x| {
            let c1 = *x as char;
            println!("{}", vowels.contains(&c1));
        });
        println!("{:?}", 1);
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let s1 = "eleetminicoworoep".to_string();
        let len = Solution::find_the_longest_substring(s1);
        assert_eq!(len, 13);
    }
}
