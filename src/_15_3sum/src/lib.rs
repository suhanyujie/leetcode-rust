/// 题目地址：https://leetcode-cn.com/problems/3sum/

/**
给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。

注意：答案中不可以包含重复的三元组。

示例：

```
给定数组 nums = [-1, 0, 1, 2, -1, -4]，

满足要求的三元组集合为：
[
  [-1, 0, 1],
  [-1, -1, 2]
]
```

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/3sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


额外的题目：给定任意一个非负整数，写段代码求二进制1的个数
*/
struct Solution;

impl Solution {
    // 排序 + 双指针
    // 参考写法 https://blog.csdn.net/starflyyy/article/details/106955473
    // 尝试其他解法 todo
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut num_arr = nums;
        let mut result: Vec<Vec<i32>> = vec![];
        // 先排序
        num_arr.sort();
        for i in 0..num_arr.len() {
            if num_arr[i] > 0 {
                break;
            }
            // 过滤重复的情况
            if i > 0 && num_arr[i] == num_arr[i - 1] {
                continue;
            }
            let target = -1 * &num_arr[i];
            let mut left = i + 1;
            let mut right = num_arr.len() - 1;
            while left < right {
                if num_arr[left] + num_arr[right] == target {
                    result.push(vec![num_arr[i], num_arr[left], num_arr[right]]);
                    left += 1;
                    right -= 1;
                    while left < right && num_arr[left] == num_arr[left - 1] {
                        left += 1;
                    }
                    while left < right && num_arr[right] == num_arr[right + 1] {
                        right -= 1;
                    }
                } else if num_arr[left] + num_arr[right] > target {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }

        result
    }

    // 统计 1 的个数
    // 参考 https://www.cnblogs.com/graphics/archive/2010/06/21/1752421.html
    pub fn get_num_of_1(num: u32) -> u32 {
        let mut n1 = num;
        let mut count: u32 = 0;
        if n1 & 1 == 1 {
            count = 1;
        }
        loop {
            n1 = n1 >> 1;
            if n1 <= 0 {
                break;
            }
            if n1 & 1 == 1 {
                count += 1;
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let input = vec![11, 2, -3, -7, 10, -9, 8, -6, -1, 0, -4, 5];
        // let expect = vec![
        //     vec![-9, -1, 10],
        //     vec![-7, -4, 11],
        //     vec![-7, -3, 10],
        //     vec![-7, -1, 8],
        //     vec![-7, 2, 5],
        //     vec![-6, -4, 10],
        //     vec![-4, -1, 5],
        // ];
        // assert_eq!(expect, Solution::three_sum(input));
        let expect = vec![vec![0, 0, 0]];
        assert_eq!(expect, Solution::three_sum(vec![0, 0, 0, 0]));
        assert!(false);
    }

    #[test]
    fn test_get_num_of_1() {
        assert_eq!(1, Solution::get_num_of_1(1));
        assert_eq!(1, Solution::get_num_of_1(2));
        assert_eq!(2, Solution::get_num_of_1(3));
        assert_eq!(1, Solution::get_num_of_1(4));
        assert_eq!(4, Solution::get_num_of_1(15));
    }
}
