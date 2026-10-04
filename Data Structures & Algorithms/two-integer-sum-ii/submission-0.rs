impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();

        for (i, &num) in numbers.iter().enumerate() {
            let x = target - num;
            if let Some(&j) = mp.get(&x) {
                return vec![(j +1), (i + 1) as i32];
            } 
            mp.insert(num, i as i32);
        }

        return vec![];
    }
}
