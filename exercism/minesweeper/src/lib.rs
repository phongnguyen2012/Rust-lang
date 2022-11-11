pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..minefield.len() {
        v.push(String::new());
        let chars = minefield[i].as_bytes();
        let start_line = if i > 0 {
            i - 1
        } else {
            i
        };
        let end_line = if i < minefield.len() - 1 {
            i + 1
        } else {
            i
        };
        for j in 0..chars.len() {
            if chars[j] == '*' as u8 {
                v[i].push_str("*");
            } else {
                let mut mine_count = 0;
                let start_col = if j == 0 {
                    j
                } else {
                    j - 1
                };
                let end_col = if j < chars.len() - 1 {
                    j + 1
                } else {
                    j
                };
                for x in start_line..=end_line {
                    let line = minefield[x].as_bytes();
                    for y in start_col..=end_col {
                        if line[y] == '*' as u8 {
                            mine_count += 1;
                        }
                    }
                }
                println!("{mine_count}");
                if mine_count > 0 {
                    v[i].push_str(&mine_count.to_string());
                } else {
                    v[i].push(chars[j] as char);
                }
            }
        }
    }
    v
}
//additional test cases