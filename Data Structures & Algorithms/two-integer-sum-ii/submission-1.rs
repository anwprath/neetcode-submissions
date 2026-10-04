impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0 as usize;
        let mut r = nums.len() - 1 ;

        while l < r {
            if target > nums[l] + nums[r] {
                l += 1;
                continue;
            }
            if target < nums[l] + nums[r] {
                r -= 1;
                continue;
            }

            if target == nums[l] + nums[r] {
                return vec![(l+1) as i32, (r+1) as i32];
            }
        }

        return vec![];
    }
}
