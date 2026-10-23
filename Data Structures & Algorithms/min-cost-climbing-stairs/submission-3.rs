impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut arr =  vec![-1 as i32; cost.len()];
        let a = Self::solve(0, &mut arr, &cost);
        let b = Self::solve(1, &mut arr, &cost);
        a.min(b)
    }

    pub fn solve(i: usize, arr: &mut [i32],  cost: &Vec<i32>) -> i32 {
        if i >= cost.len()  {
            return 0;
        }
        if arr[i] != -1 {
            return arr[i];
        }
        let a = Self::solve(i+1, arr, cost);
        let b = Self::solve(i+2, arr, cost);

        arr[i] = cost[i] + a.min(b);

        arr[i]
    }
}
