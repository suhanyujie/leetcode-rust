//! 题目地址：https://leetcode-cn.com/problems/container-with-most-water/

struct Solution {}

impl Solution {
    /// 能否容纳更多的水取决于短板，
    /// 容器边有三个，实际上可以求矩形面积的方式得到“容积”
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_volume: i32 = 0;
        for i in 0..height.len() {
            let first_height = height[i];
            if first_height < 1 {
                continue;
            }
            height.iter().enumerate().for_each(|(index, val)|{
                if index == i || *val < 1 {
                    return ();
                } else {
                    let bottom_diff = if index > i {index - &i} else {i-index};
                    let useful_height = if first_height > *val {val} else {&first_height};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(49, Solution::max_area(arr));
    }
}
