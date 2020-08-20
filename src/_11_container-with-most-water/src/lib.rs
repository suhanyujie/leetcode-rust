//! 题目地址：https://leetcode-cn.com/problems/container-with-most-water/

struct Solution {}

impl Solution {
    /// 能否容纳更多的水取决于短板，
    /// 容器边有三个，实际上可以求矩形面积的方式得到“容积”
    pub fn user1_max_area(height: Vec<i32>) -> i32 {
        let mut max_volume: i32 = 0;
        for i in 0..height.len() {
            let first_height = height[i];
            if first_height < 1 {
                continue;
            }
            height.iter().enumerate().for_each(|(index, val)| {
                if index == i || *val < 1 {
                    ()
                } else {
                    let bottom_diff = if index > i { index - i } else { i - index };
                    let useful_height = if first_height > *val {
                        val
                    } else {
                        &first_height
                    };
                    let res = (bottom_diff as i32) * (*useful_height);
                    if res > max_volume {
                        max_volume = res;
                    }
                }
            });
        }
        max_volume
    }
}

impl Solution {
    /// 优化版，参考官方题解：https://leetcode-cn.com/problems/container-with-most-water/solution/
    /// 该方法，时间复杂度： O(N)
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_volume: i32 = 0;
        let mut l: i32 = 0;
        let mut r: i32 = (height.len() - 1) as i32;
        loop {
            let res: i32 = Solution::min(height[l as usize], height[r as usize]) * (r - l);
            if max_volume < res {
                max_volume = res;
            }
            if height[l as usize] < height[r as usize] {
                l += 1;
            } else {
                r -= 1;
            }
            if l >= (height.len() as i32) || r <= 0 {
                break;
            }
        }

        max_volume
    }

    fn min(num1: i32, num2: i32) -> i32 {
        if num1 < num2 {
            num1
        } else {
            num2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(49, Solution::max_area(arr));
    }
}
