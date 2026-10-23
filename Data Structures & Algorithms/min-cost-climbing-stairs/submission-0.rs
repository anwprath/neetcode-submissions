impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let a = Self::solve(0, &mut vec![-1; cost.len()], &cost);
        let b = Self::solve(1, &mut vec![-1; cost.len()], &cost);
        return min(a, b);
    }

    pub fn solve(i: usize, arr: &mut Vec<i32>,  cost: &Vec<i32>) -> i32 {
        if i >= cost.len()  {
            return 0;
        }
        if arr[i] != -1 {
            return arr[i];
        }
        let a = cost[i] + Self::solve(i+1, arr, cost);
        let b = cost[i] + Self::solve(i+2, arr, cost);

        if a < b {
            arr[i] = a;
        } else {
            arr[i] = b;
        }

        return arr[i];
    }
}
