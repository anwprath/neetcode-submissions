impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut l = 0i32;
        let mut r = (s.len() - 1) as i32;


        while l < r {
            while l < r && !s[l as usize].is_ascii_alphanumeric() {
                l += 1;
            }
            while l < r && !s[r as usize].is_ascii_alphanumeric() {
                r -= 1;
            }
            if s[l as usize].to_ascii_lowercase() != s[r as usize].to_ascii_lowercase() {
                return false;
            }
            l += 1;
            r -=1;
        }
        true
    }
}
