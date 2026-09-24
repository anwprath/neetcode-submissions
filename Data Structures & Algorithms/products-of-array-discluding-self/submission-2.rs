impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        let mut prefix = 1;
        for i in 0..n {
            res[i] = prefix;
            prefix = prefix * nums[i];
        }

        let mut postfix = 1;
        for i in (0..n).rev() {
            res[i] = res[i] * postfix;
            postfix = postfix*nums[i];
        }
        res
    }
}
