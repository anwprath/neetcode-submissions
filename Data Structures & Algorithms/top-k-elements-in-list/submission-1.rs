impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        let mut freqVector = vec![vec![]; nums.len()+1];

        for num in nums {
            let count = mp.entry(num).or_insert(0);
            *count += 1;
        }
        for (num, cnt) in mp {
            freqVector[cnt].push(num);
        }

        let mut ans = Vec::new();
        for i in (1..freqVector.len()).rev() {
            for &num in &freqVector[i] {
                ans.push(num);
                if ans.len() == (k as usize) {
                    return ans;
                }
            }
        } 

        return ans;
    }
}
