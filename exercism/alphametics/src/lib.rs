use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Eq, Clone, Copy)]
struct Node {
    ch: char,
    weight: i64,
    leading: bool,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ch.cmp(&other.ch)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.ch == other.ch
    }
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut char_params = parse_string(input);
    char_params.sort();
    let mut ans = vec![0u8; char_params.len()];
    //perform recursion search in depth
    dfs(0, &mut ans, &char_params, &(0..=9).collect())
}
fn dfs(
    depth: usize,
    ans: &mut Vec<u8>,
    char_params: &Vec<Node>,
    digits: &Vec<u8>,
) -> Option<HashMap<char, u8>> {
    if depth == char_params.len() {
        return if check_nums(char_params, ans) {
            Some(
                char_params
                    .iter()
                    .zip(ans.iter())
                    .map(|(&node, &d)| (node.ch, d))
                    .collect(),
            )
        } else {
            None
        };
    }
    for (i, &digit) in digits.iter().enumerate() {
        if 0 == digit && char_params[depth].leading {
            continue;
        }
        ans[depth] = digit;
        let mut digits2 = digits.clone();
        digits2.remove(i);
        if let Some(x) = dfs(depth + 1, ans, char_params, &digits2) {
            return Some(x);
        }
    }
    None
}
fn parse_string(input: &str) -> Vec<Node> {
    let mut leading_chars = HashSet::new();
    let mut ans = HashMap::new();
    let mut pos_radix = -1;
    let mut prev_char = ' ';
    //lets begin from sum result with negative weight
    for c in input.chars().rev() {
        match c {
            'A'..='Z' => {
                *ans.entry(c).or_insert(0i64) += pos_radix;
                pos_radix *= 10;
                prev_char = c;
            }
            _ => {
                pos_radix = 1;
                leading_chars.insert(prev_char);
            }
        }
    }
    leading_chars.insert(prev_char);
    ans.iter()
        .map(|(&c, &v)| Node {
            ch: c,
            weight: v,
            leading: leading_chars.contains(&c),
        })
        .collect()
}
fn check_nums(chars: &Vec<Node>, digits: &Vec<u8>) -> bool {
    //check equation with chars wight
    let mut sum = 0;
    for (i, &digit) in digits.iter().enumerate() {
        sum += digit as i64 * chars[i].weight;
    }
    0 == sum
}