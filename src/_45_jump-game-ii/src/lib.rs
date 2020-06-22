/// 题目地址 https://leetcode-cn.com/problems/jump-game-ii/

struct Solution;

/*
输入: [2,3,1,1,4]
输出: 2
解释: 跳到最后一个位置的最小跳跃数是 2。
从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/jump-game-ii
*/
/// 老实说，做这题之前，我还不知道什么是动态规划，因此对这题的我第一思路是直接暴力破解，但是看了「力扣」的题解，对动态规划才有了一丝丝的理解

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut position = (nums.len() -1) as i32 ;
        let mut steps = 0;
        while position > 0 {
            for index in 0..position {
                // 当前值 + index  = 最后一个值的索引，则表示可以到达最后一个点
                if nums[index as usize] + index >= position  {
                    position = index;
                    steps += 1;
                    break;
                } 
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump1() {
        let input = vec![2, 3, 1, 1, 4];
        let output = Solution::jump(input);
        println!("the result is:{}", output);
        assert_eq!(2, output);
    }
}
