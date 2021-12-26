//! 题目地址：https://leetcode-cn.com/problems/two-sum/

use std::vec;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 先对数组排序
        let mut nums = nums;
        nums.sort();
        let mut result: Vec<i32> = vec![];
        for i in 0..nums.len() {
            if nums[i] + nums[j] > target {

            }if nums[i] + nums[j] < target {

            } else {

            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
