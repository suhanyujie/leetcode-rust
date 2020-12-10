/// 四数之和
/// 题目地址 https://leetcode-cn.com/problems/4sum/

/*
给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d ，使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。

注意：

答案中不可以包含重复的四元组。

示例：

给定数组 nums = [1, 0, -1, 0, -2, 2]，和 target = 0。

满足要求的四元组集合为：
[
  [-1,  0, 0, 1],
  [-2, -1, 1, 2],
  [-2,  0, 0, 2]
]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/4sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

struct Solution {}

impl Solution {
    // 排序 + 双指针
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() <= 3 {
            return vec![];
        }
        let mut arr = nums.clone();
        // 排序
        arr.sort();
        // 双指针
        let mut result: Vec<Vec<i32>> = vec![];
        for a in 0..arr.len() {
            if a > 0 && arr[a] == arr[a - 1] {
                continue;
            }
            for b in a + 1..arr.len() {
                if b > 0 && arr[b] == arr[b - 1] {
                    continue;
                }
                let mut left = b + 1;
                let mut right = arr.len() - 1;
                while left < right {
                    let add_res = arr[a] + arr[b] + arr[left] + arr[right];
                    if add_res == target {
                        result.push(vec![arr[a], arr[b], arr[left], arr[right]]);
                        // 相等之后，因为结果要求不能是相同的三元组，因此需要继续下一组数的判断。
                        left += 1;
                        right -= 1;
                        while left < right && arr[left] == arr[left - 1] {
                            left += 1;
                        }
                        while left < right && arr[right] == arr[right + 1] {
                            right -= 1;
                        }
                    } else if add_res > target {
                        right -= 1;
                    } else if add_res < target {
                        left += 1;
                    }
                }
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
        let expect_val = vec![vec![-2, -1, 1, 2],vec![-2, 0, 0, 2], vec![-1, 0, 0, 1],];
        assert_eq!(expect_val, Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0));
        let expect_val = vec![vec![0, 0, 0, 0],];
        assert_eq!(expect_val, Solution::four_sum(vec![0, 0, 0, 0], 0));
    }
}
