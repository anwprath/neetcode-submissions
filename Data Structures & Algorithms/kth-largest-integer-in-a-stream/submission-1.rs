use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
	nums: Vec<i32>,
	k: i32,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
		return KthLargest{k:k, nums: nums};
    }

    pub fn add(&mut self, val: i32) -> i32 {
		self.nums.push(val);
		self.nums.sort();
		return self.nums[self.nums.len() - self.k as usize];
    }
}
