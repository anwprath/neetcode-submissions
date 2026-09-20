func hasDuplicate(nums []int) bool {
    mp := make(map[int]struct{})
	for i := range nums {
		mp[nums[i]] = struct{}{}
	}
	return len(mp) < len(nums)
}
