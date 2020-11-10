/// 题目地址：https://leetcode-cn.com/problems/3sum-closest/
/*
给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。返回这三个数的和。假定每组输入只存在唯一答案。
示例：

输入：nums = [-1,2,1,-4], target = 1
输出：2
解释：与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/3sum-closest
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

struct Solution;

impl Solution {
    // 排序 + 双指针
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut num_arr = nums;
        num_arr.sort();
        // 先找到一组和，作为比较的对象
        let mut some_close = num_arr[0] + num_arr[1] + num_arr[2];
        let diff_abs = some_close - target;
        let mut diff_abs = if diff_abs > 0 {
            diff_abs
        } else {
            diff_abs * -1
        };
        for i in 0..num_arr.len() {
            if i > 0 && num_arr[i] == num_arr[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = num_arr.len() - 1;
            while left < right {
                let tmp_res = num_arr[i] + num_arr[left] + num_arr[right];
                if tmp_res == target {
                    // 如果找到值的和跟 target 一样的三个数，则说明已找到，直接退出循环。
                    some_close = tmp_res;
                    diff_abs = 0;
                    break;
                } else if tmp_res > target {
                    right -= 1;
                } else if tmp_res < target {
                    left += 1;
                }
                let diff = tmp_res - target;
                let tmp_diff_abs = if diff > 0 { diff } else { diff * -1 };
                if tmp_diff_abs < diff_abs {
                    diff_abs = tmp_diff_abs;
                    some_close = tmp_res;
                }
            }
        }
        some_close
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::three_sum_closest(vec![1,1,-1,-1,3], 1));
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
        assert_eq!(-1, Solution::three_sum_closest(vec![1,1,-1,-1,3], -1));
        assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0, 0], 1));
    }
}
