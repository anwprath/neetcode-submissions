impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; 3];
        if n <= 2 {
            return if n == 1 { 0 } else { cost[0].min(cost[1]) };
        }
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..n{
            dp[2] = cost[i] + (dp[1]).min(dp[0]);
            dp[0] = dp[1];
            dp[1] = dp[2];
        }
        dp[1].min(dp[0])
    }
}
