impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut one=1;
        let mut two=0;

        for _ in 0..n {
            let three = one;
            one = one + two;
            two = three;
        }
        one
    }
}
