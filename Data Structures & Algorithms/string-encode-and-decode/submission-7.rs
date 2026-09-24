impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = format!("{}:", strs.len());

        for s in &strs {
            result.push_str(&format!("{}.", s.len()));
        }
        result.push(';');

        for s in &strs {
            result.push_str(s);
        }
        // print!("Final encoded string: {}\n", result);
        return result;
    }

    pub fn decode(s: String) -> Vec<String> {
        let colon_pos = s.find(":").unwrap();
        let n : usize = s[0..colon_pos].parse().unwrap();

        let bytes = s.as_bytes();

        let mut i = colon_pos +1;
        let mut curr_num  = 0;
        let mut lengths = Vec::new();

        while i < bytes.len() {
            match bytes[i] {
                b'.' => {
                    lengths.push(curr_num);
                    // print!("Lengths vector: {:?}\n", lengths);
                    curr_num = 0;
                }
                b';' => {
                    break;
                }
                b'0'..=b'9' => {
                    curr_num = curr_num * 10 + ((bytes[i] - b'0') as usize);
                    // print!("curr_num: {}, i: {}\n", curr_num, i);
                }
                _ => {}
            }
            i += 1;
        }


        let mut start = i + 1;
        let mut result = Vec::new();

        for &len in &lengths {
            let end = start + len;
            result.push(s[start..end].to_string());
            start = end;
        }
        result
    }
}
