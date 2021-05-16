//! 题目名称：编辑距离
//! 题目地址：https://leetcode-cn.com/problems/edit-distance/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let l1 = word1.len();
        let l2 = word2.len();
        // 实例化一个二维数组
        let mut v_dp: Vec<Vec<i32>> = Vec::with_capacity(l1);
        for index in 0..l1 {
            let mut col_arr = Vec::with_capacity(l2);
            for _ in 0..l2 {
                col_arr.push(0);
            }
            v_dp.push(col_arr);
        }
        // 初始化每一列第一行的值，除了第一个
        for col in 1..l2 {
            v_dp[0][col] = col as i32;
        }
        // 初始化每一行第一列的值的值，除了第一个
        for row in 1..l1 {
            v_dp[row][0] = row as i32;
        }
        let c1_arr: Vec<char> = word1.chars().collect();
        let c2_arr: Vec<char> = word2.chars().collect();
        for row in 1..l1 {
            for col in 1..l2 {
                if c1_arr[row - 1] == c2_arr[col - 1] {
                    v_dp[row][col] = v_dp[row - 1][col - 1];
                } else {
                    v_dp[row][col] = Solution::min(
                        v_dp[row - 1][col - 1],
                        v_dp[row - 1][col],
                        v_dp[row][col - 1],
                    ) + 1;
                }
            }
        }

        return v_dp[l1 - 1][l2 - 1];
    }

    pub fn min(a: i32, b: i32, c: i32) -> i32 {
        let min_num = a.min(b);
        min_num.min(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance("".into(), "".into()), 0);
        assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
    }

    #[test]
    fn test_chars() {
        let s1 = String::from("你是谁呢？");
        let arr: Vec<char> = s1.chars().collect();
        println!("{:?}", arr);
    }
}
