impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        return Self::solve(n, &mut HashMap::new());
    }
    pub fn solve(n: i32, mp: &mut HashMap<i32, i32>) -> i32 {
        if n < 0 {
            return 0;
        }
        if n == 0 {
            return 1;
        }
        if let Some(&x) = mp.get(&n) {
            return x;
        }
        let res = Self::solve(n - 1, mp) + Self::solve(n - 2, mp);
        mp.insert(n,res);
        return *mp.get(&n).unwrap();
    }
}
