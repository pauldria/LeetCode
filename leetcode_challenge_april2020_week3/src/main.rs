fn main() {
    let v = vec![5,4];
    println!("{:?}", v);
    println!(" --> {:?}", SolutionProductExceptSelf::product_except_self(v));

    //let s = String::from("(())((())()()(*)(*()(())())())()()((()())((()))(*");
    let s = String::from("((*)(*))((*");
    //let s = String::from("(*))");
    println!("{}", s);
    println!(" --> {}", SolutionValidParens::check_valid_string(s));

}

struct SolutionProductExceptSelf { }

impl SolutionProductExceptSelf {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n <= 1 {
            return vec![1];
        }
        let mut result: Vec<i32> = vec![1; n];
        for i in (0..n-1).rev() {
            match nums.get(i) {
                None => (),
                Some(val) => {
                    result[i] = nums[i+1]*result[i+1];
                }
            }
        }
        let mut rolling_product = 1;
        for i in (1..n) {
            rolling_product *= nums[i-1];
            match result.get(i) {
                None => (),
                Some(val) => {
                    result[i] = val*rolling_product;
                }
            }
        }
        return result;
    }
}

struct SolutionValidParens { }

impl SolutionValidParens {
    pub fn check_valid_string(s: String) -> bool {
        let n = s.len();
        let mut rolling_sum: Vec<i32> = vec![0; n];
        let mut num_stars: Vec<i32>   = vec![0; n];
        let mut max_pos_needed = 0;
        if n == 0 {
            return true;
        }
        else if n == 1 {
            return (s.as_str() == "*");
        }

        // Start the vectors
        match s.chars().next().unwrap() {
            '(' => {
                rolling_sum[0] = 1;
            },
            ')' => {
                rolling_sum[0] = -1;
            },
            '*' => {
                num_stars[0] = 1;
            }
            _ => ()
        }

        if rolling_sum[0] < 0 {
            // We have enough stars to compensate, but we need to track
            if num_stars[0] >= -1*rolling_sum[0] {
                let disc = num_stars[0] + rolling_sum[0];
                max_pos_needed = if disc > max_pos_needed { disc } else { max_pos_needed};
            }
            else {
                return false;
            }
        }

        let mut idx: usize = 1;

        for c in s[1..n].chars() {
            // We must continue to propagate regardless!
            rolling_sum[idx] = rolling_sum[idx-1];
            num_stars[idx]   = num_stars[idx-1];
            match c {
                '(' => {
                    rolling_sum[idx] = rolling_sum[idx-1] + 1;
                },
                ')' => {
                    rolling_sum[idx] = rolling_sum[idx-1] - 1;
                },
                '*' => {
                    num_stars[idx]   = num_stars[idx-1]  + 1;
                }
                _ => ()
            }
            println!("{} {:?} {:?}", idx, rolling_sum, num_stars);
            if rolling_sum[idx] < 0 {
                // We have enough stars to compensate, but we need to track
                if num_stars[idx] >= -1*rolling_sum[idx] {
                    let disc = num_stars[idx] + rolling_sum[idx];
                    max_pos_needed = if disc > max_pos_needed
                                        { disc }
                                     else
                                        { max_pos_needed };
                }
                else {
                    return false;
                }
            }

            idx += 1;
        }
        // Now, we need to make sure that we can reconcile everything.
        // We need to understand how many * we had to turn into (, and then
        // see if we have enough * left to turn into )
        // At this point, we know rolling_sum[n-1] >= 0
        return num_stars[n-1] - max_pos_needed >= rolling_sum[n-1];
    }
}