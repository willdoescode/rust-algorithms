use std::collections::HashMap;

fn main() {
    println!("{:?}", twosum(vec![2, 7, 11, 15], 9));
    println!("{}", count_upper(String::from("HdjsfjsJ")));
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

