//! 题目名称：70. 爬楼梯
//! 题目地址 https://leetcode-cn.com/problems/climbing-stairs/

/*
假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

注意：给定 n 是一个正整数。

示例 1：

输入： 2
输出： 2
解释： 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶
示例 2：

输入： 3
输出： 3
解释： 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/climbing-stairs
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

struct Solution;

/*
// 先观察观察，的确是斐波那契数列（看来题解之后才知道）。
1->1
2->2
3->3
4->5
...

确定是斐波那契数列之后，就好办了，转换为熟悉的算法了。根据斐波那契数列规则，有：`dp[n] = dp[n-1] + dp[n-2]`

*/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let dp: Vec<i32> = vec![];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            dp[n] = dp[i-1] + dp[i-2];
        }
        return dp[n];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
