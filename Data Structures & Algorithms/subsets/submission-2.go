func subsets(nums []int) [][]int {
    var ans [][]int = [][]int{[]int{}}
    for i := range nums {
        ans = apply(ans, nums[i], true)
        ans = apply(ans, nums[i], false)
    }
    return ans
}

func apply(ans [][]int, num int, take bool) [][]int {
    if take {
        var temp [][]int
        for i := range ans {
            var x []int = make([]int, len(ans[i]))
            copy(x, ans[i])
            x = append(x, num)
            temp = append(temp, x)
        }
        ans = append(ans, temp...)
    }
    return ans
}