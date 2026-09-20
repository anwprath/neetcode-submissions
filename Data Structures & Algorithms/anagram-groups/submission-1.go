

func groupAnagrams(strs []string) [][]string {
	mp := make(map[[26]int][]string)

	for _, s := range strs {
		var t [26]int
		for _, c := range s {
			t[c-'a']++
		}

		mp[t] = append(mp[t], s)
	}

	ans := make([][]string, 0)
	for _, t := range mp {
		ans = append(ans, t)
	}
	return ans
}
