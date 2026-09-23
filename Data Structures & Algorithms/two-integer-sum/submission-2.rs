impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            let diff = target - n;
            if let Some(&j) = mp.get(&diff) {
                if i != j {
                    return vec![j as i32, i as i32];
                }
            }
            mp.insert(n, i);
        }
        
        vec![]
    }
}
