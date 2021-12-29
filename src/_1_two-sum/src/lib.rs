//! 题目地址：https://leetcode-cn.com/problems/two-sum/

use std::{collections::HashMap, hash::Hash, vec};

struct Solution;
impl Solution {
    /// 通过 map 的方式，将时间复杂度降为 O(N)，但空间复杂度也增加为 `O(N)`
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut map1: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let tmp_key = &((target - num) as i32);
            let tmp_res = map1.get(tmp_key);
            if tmp_res.is_none() {
                map1.insert(*num,i);
            } else {
                result.push(*tmp_res.unwrap() as i32);
                result.push(i as i32);
                break;
            }
        }

        result
    }

    /// 暴力破解法，将所有可能的组合进行比较。时间复杂度 O(N^2)
    /// https://leetcode-cn.com/problems/two-sum/solution/liang-shu-zhi-he-by-leetcode-solution/
    pub fn two_sum_01(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 先对数组排序
        let mut nums = nums;
        nums.sort();
        let mut result: Vec<i32> = vec![];
        'outer: for i in 0..nums.len() {
            let mut j = i + 1;
            while j < nums.len() {
                if nums[i] + nums[j] == target {
                    result.push(i as i32);
                    result.push(j as i32);
                    break 'outer;
                }
                j += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum_01(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
