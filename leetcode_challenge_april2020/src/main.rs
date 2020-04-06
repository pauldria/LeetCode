// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/528/week-1/3283/

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use std::i32;

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

    let v1 = vec![-2, 1, -3];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let v1 = vec![-2,1,-3,4,-1,2,1,-5,4];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let v1 = vec![-2, -1];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let v1 = vec![-1, -2];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let v1 = vec![1, 2];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let v1 = vec![-2, -3, -1];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let v1 = vec![8, -19, 5, -4, 20];

    println!("Array: {:?}", v1);
    println!("Backward rolling sum is {:?}", SolutionMaximumSubarray::backward_rolling_sum(&v1));
    println!("Max-backward is {:?}", SolutionMaximumSubarray::min_backward(&v1));
    println!("Max-subarray is {}", SolutionMaximumSubarray::max_sub_array(v1));

    let mut v1 = vec![0,1,0,3,12];
    println!("Array: {:?}", v1);
    SolutionMoveZeroes::move_zeroes(&mut v1);
    println!("  ---> {:?}", v1);

    let v1 = vec![7,6,4,3,1];
    println!("Array: {:?}", v1);
    let profit = SolutionBuySell::max_profit(v1);
    println!("  ---> {}", profit);

    let s = "samantha";
    let mut s_chars: Vec<char> = s.chars().collect::<Vec<char>>();
    s_chars.sort();
    let s_sorted: String = s_chars.iter().collect();
    println!("{} --> {}", s, s_sorted);

    let v1: Vec<String> = vec![String::from("eat"),
                               String::from("tea"),
                               String::from("tan"),
                               String::from("ate"),
                               String::from("nat"),
                               String::from("bat")];
    println!("{:?}", v1);
    let v1_grouped = SolutionGroupAnagrams::group_anagrams(v1);
    println!("{:?}", v1_grouped);

    let v1: Vec<String> = vec![String::from("eat")];
    println!("{:?}", v1);
    let v1_grouped = SolutionGroupAnagrams::group_anagrams(v1);
    println!("{:?}", v1_grouped);

    let v1: Vec<String> = vec![String::from(""),
                               String::from("")];
    println!("{:?}", v1);
    let v1_grouped = SolutionGroupAnagrams::group_anagrams(v1);
    println!("{:?}", v1_grouped);

    let v1: Vec<String> = Vec::new();
    println!("{:?}", v1);
    let v1_grouped = SolutionGroupAnagrams::group_anagrams(v1);
    println!("{:?}", v1_grouped);
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

struct SolutionMaximumSubarray {}

impl SolutionMaximumSubarray {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        }

        let min_backward = SolutionMaximumSubarray::min_backward(&nums);
        let v_sum: i32 = nums.iter().sum();

        // Edge case: if we choose nothing in the beginning, then
        // we look at max_backward[1] or 0 if we choose to pick nothing
        let mut min_sofar = min(0, min_backward[1]);

        let mut i = 0;
        let mut rolling_sum = 0;

        while i < n - 2 {
            println!("i={}, min_sofar={}", i, min_sofar);
            rolling_sum += nums[i];
            let min_candidate = rolling_sum + min_backward[i + 2];
            if min_candidate < min_sofar {
                min_sofar = min_candidate;
            }
            i += 1;
        }

        // Edge case: at the end, we can't utilize the max_backward and only can look at
        // rolling sum. Note the duality - here we do NOT use max_backward.
        rolling_sum += nums[i];
        println!("i={}, min_sofar={}", i, min_sofar);
        if rolling_sum < min_sofar {
            min_sofar = rolling_sum;
        }

        return v_sum - min_sofar;
    }

    // We could also choose to pick nothing!
    pub fn min_backward(v: &Vec<i32>) -> Vec<i32> {
        let v_brs = SolutionMaximumSubarray::backward_rolling_sum(v);
        let n = v.len();
        let mut newv = vec![0; n];
        let mut i = n-1;
        newv[i] = min(0, v[i]);
        i -= 1;
        while i > 0 {
            newv[i] = min(newv[i+1], v_brs[i]);
            i -= 1;
        }
        newv[i] = min(newv[i+1], v_brs[i]);
        return newv;
    }

    pub fn backward_rolling_sum(v: &Vec<i32>) -> Vec<i32> {
        let n      = v.len();
        let mut newv = vec![0; n];
        let mut i    = n - 1;
        newv[i] = v[i];
        i -= 1;
        while i > 0 {
            newv[i] = newv[i + 1] + v[i];
            i -= 1;
        }
        newv[i] = newv[i + 1] + v[i];
        return newv;
    }
}

struct SolutionMoveZeroes {}

impl SolutionMoveZeroes {
    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) -> () {
        let tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) -> () {
        let n = nums.len();

        // Base case
        if n < 2 {
            return
        }

        // ptr is what we use to walk through vector
        // fz will be location of first zero in the set of zeroes we're tracking
        // lz will be the location of the last zero in the set of zeroes we're tracking
        let mut ptr = 0;
        let mut fz = n;
        let mut lz = n;


        while ptr < n {
            if nums[ptr] == 0 {
                if lz == n {
                    fz = ptr;
                    lz = ptr;
                }
                else {
                    lz = ptr;
                }
            }
            else {                  // nums[ptr] != 0
                if lz == n {}
                else {
                    SolutionMoveZeroes::swap(nums, fz, ptr);
                    fz += 1;
                    lz += 1;
                }
            }
            ptr += 1;
        }
    }
}

struct SolutionBuySell {}

impl SolutionBuySell {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let n = prices.len();
        if n <= 1 {
            return 0;
        }

        let mut total_profit = 0;

        let mut is_buying = false;
        let mut bought_price = 0;
        // Do the base case - we only buy if the next price is higher
        if prices[1] > prices[0] {
            is_buying = true;
            bought_price = prices[0];
        }

        let mut i = 1;
        while i < n-1 {
            if is_buying {
                if prices[i+1] < prices[i] {
                    is_buying = false;
                    total_profit += prices[i] - bought_price;
                }
                else { }                        // Do nothing, price is still increasing
            }
            else {
                if prices[i+1] > prices[i] {
                    is_buying = true;
                    bought_price = prices[i];
                }
                else { }                        // Do nothing, price is still decreasing
            }
            i += 1;
        }
        // Don't forget the end!
        if is_buying {
            total_profit += prices[i] - bought_price;
        }
        return total_profit;
    }
}

struct SolutionGroupAnagrams {}

impl SolutionGroupAnagrams {
    pub fn sort_string(str:String) -> String {
        let mut s_chars: Vec<char> = str.chars().collect::<Vec<char>>();
        s_chars.sort();
        return s_chars.iter().collect();
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut canonical: Vec<(String, String)> = Vec::new();

        // First, we map each string into its canonical form
        // What do we do with duplicates???
        for s in strs.into_iter() {
            let s_key = s.clone();
            canonical.push((s_key, SolutionGroupAnagrams::sort_string(s)));
        }
        let mut dict_grouped: HashMap<&String, Vec<String>> = HashMap::new();

        for (k, v) in canonical.iter() {
            let k_val = k.clone();
            let entry = dict_grouped.get(v);

            // If we don't have a key available, we initialize with a new
            if entry.is_none() {
                dict_grouped.insert(v, Vec::new());
            }
            else { }
            let foo = dict_grouped.get_mut(v);
            match foo {
                None => (),
                Some(val) => val.push(k_val)
            }
        }

        let mut grouped_anagrams: Vec<Vec<String>> = Vec::new();
        for (_k, v) in dict_grouped.into_iter() {
            grouped_anagrams.push(v);
        }

        return grouped_anagrams;
    }
}