//! 题目地址：https://leetcode-cn.com/problems/two-sum/

use std::vec;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 先对数组排序
        let mut nums = nums;
        nums.sort();
        let mut result: Vec<i32> = vec![];
        let mut j = nums.len() - 1;
        let mut i = 0;
        while i != j {
            if nums[i] + nums[j] > target {
                j -= 1;
            }
            if nums[i] + nums[j] < target {
                i += 1;
            } else {
                result.push(i as i32);
                result.push(j as i32);
                break;
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
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
