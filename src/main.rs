use std::collections::HashMap;

fn main() {
  println!("{:?}", twosum(vec![2, 7, 11, 15], 9));
  println!("{}", count_upper(String::from("HdjsfjsJ")));
  println!("{:?}", reverse_string(&mut vec!['h', 'e', 'l', 'l', 'o']));
  println!("{}", remove_outer_parentheses(String::from("(()())(())")));
  println!("{}", reverse_str(String::from("abcdefg"), 2));
}

fn twosum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut known: HashMap<i32, i32> = HashMap::new();
  for (index, value) in nums.iter().enumerate() {
    if known.contains_key(value) {
      return vec![known[value] as i32, index as i32];
    } else {
      known.insert(target - *value, index as i32);
    }
  }
  vec![0, 0]
}

fn count_upper(s: String) -> i32 {
  let mut count = 0;
  for c in s.chars() {
    if c.is_uppercase() {
      count += 1;
    }
  }
  count
}

fn reverse_string(s: &mut Vec<char>) -> Vec<char> {
  let mut new: Vec<char> = Vec::new();
  while let Some(top) = s.pop() {
    new.push(top)
  }
  new
}

fn remove_outer_parentheses(s: String) -> String {
  let mut re = String::new();
  let mut depth = 0;
  for c in s.chars() {
    if c == ')' {
      depth -= 1;
    }
    if depth != 0 {
      re.push_str(&c.to_string());
    }
    if c == '(' {
      depth += 1;
    }
  }
  re
}

// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Reverse String II.
// Memory Usage: 2.1 MB, less than 100.00% of Rust online submissions for Reverse String II.

fn reverse_str(s: String, k: i32) -> String {
  let mut a = s.chars().collect::<Vec<char>>();
  let n = a.len();
  for start in (0..n).step_by((k + k) as usize) {
    let mut i = start;
    let mut j = std::cmp::min(start + k as usize - 1, n - 1);
    while i < j {
      a.swap(i, j);
      i += 1;
      j -= 1
    }
  }
  a.iter().collect::<String>()
}
