

func groupAnagrams(strs []string) [][]string {
	mp := make(map[string][]string)
	for _, s := range strs {
		chars := strings.Split(s, "")
		sort.Strings(chars)
		sorted := strings.Join(chars, "")

		mp[sorted] = append(mp[sorted], s)
	}

	ans := make([][]string, 0)
	for _, t := range mp {
		ans = append(ans, t)
	}
	return ans
}
