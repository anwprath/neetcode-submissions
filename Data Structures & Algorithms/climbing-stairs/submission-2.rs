impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut arr = vec![0; (n + 1) as usize];
        arr[1] = 1;
        arr[2] = 2;
        for i in 3..=(n as usize) {
            arr[i] = arr[i-1] + arr[i-2];
        }
        arr[n as usize]
    }
}
