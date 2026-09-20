func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	mp := make(map[byte]int)
	for i := 0; i < len(s) ; i++ {
		mp[s[i]]++
	}
	for i := 0; i < len(t) ; i++ {
		if mp[t[i]] == 0 {
			return false
		}
		mp[t[i]]--
	}
	return true
}
