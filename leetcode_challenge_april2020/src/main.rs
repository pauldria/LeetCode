// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/528/week-1/3283/

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let v1 = vec![2, 2, 1];
    let v2 = vec![4, 1, 2, 1, 2];

    println!("The unique element in {:?} is...", v1);
    let n1 = SolutionSingleNumber::single_number(v1);
    println!("{}", n1);

    println!("The unique element in {:?} is...", v2);
    let n1 = SolutionSingleNumber::single_number(v2);
    println!("{}", n1);

    println!("5/10 calculates to . . . {}", 5/10);
    println!("5%10 calculates to . . . {}", 5%10);

    println!("Is {} a happy number? {}", 19, SolutionHappyNumber::is_happy(19));
    println!("Is {} a happy number? {}", 26, SolutionHappyNumber::is_happy(26));
}

struct SolutionSingleNumber {}

impl SolutionSingleNumber {
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

struct SolutionHappyNumber {}

impl SolutionHappyNumber {
    pub fn sum_of_squares(mut n: i32) -> i32 {
        let mut total_sum: i32 = 0;
        while n > 0 {
            total_sum += (n % 10).pow(2);
            n = n / 10;
        }
        return total_sum;
    }
    pub fn is_happy(n: i32) -> bool {
        let mut numbers_seen: HashSet<i32> = HashSet::new();
        numbers_seen.insert(n);
        let mut cur_value = n;
        loop {
            let new_value = SolutionHappyNumber::sum_of_squares(cur_value);
            if new_value == 1 {
                return true;
            }
            else if numbers_seen.contains(&new_value) {
                return false;
            }
            else {
                numbers_seen.insert(new_value);
            }
            cur_value = new_value;
        }
    }
}