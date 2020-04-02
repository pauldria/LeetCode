// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/528/week-1/3283/

use std::collections::HashMap;

fn main() {
    let v1 = vec![2, 2, 1];
    let v2 = vec![4, 1, 2, 1, 2];

    println!("The unique element in {:?} is...", v1);
    let n1 = Solution::single_number(v1);
    println!("{}", n1);

    println!("The unique element in {:?} is...", v2);
    let n1 = Solution::single_number(v2);
    println!("{}", n1);
}

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut tallies = HashMap::new();
        for n in &nums {
            let count = tallies.entry(*n).or_insert(0);
            *count += 1;
        }

        for (key, value) in &tallies {
            if *value == 1 {
                return *key;
            }
        }

        // We should never reach this point.
        panic!("You told me there would be a number that exists only once!")
    }
}