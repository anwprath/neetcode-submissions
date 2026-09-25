impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let  set : HashSet<i32>= nums.into_iter().collect();           
        let mut ans = 0;
        for &e in &set {
            if set.contains(&(e-1)) {
                continue
            }

            let mut curr = e;
            let mut length = 1;
            while set.contains(&(curr+1)) {
                length += 1;
                curr += 1;
            }
            ans = ans.max(length);
        }
        ans
            
    }
}
