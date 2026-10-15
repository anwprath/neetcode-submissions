impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]];

        fn apply(mut ans: Vec<Vec<i32>>, num: i32) -> Vec<Vec<i32>> {
            let mut temp = Vec::new();
            for subset in &ans {
                let mut x = subset.clone();
                x.push(num);
                temp.push(x);
            }
            ans.extend(temp);
            ans
        }

        for num in nums {
            ans = apply(ans, num);
        }
        ans

    }

}
