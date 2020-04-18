use std::cmp::min;
use std::collections::HashSet;

fn main() {
    let v = vec![5,4];
    println!("{:?}", v);
    println!(" --> {:?}", SolutionProductExceptSelf::product_except_self(v));

    let s = String::from("(())((())()()(*)(*()(())())())()()((()())((()))(*");
    //let s = String::from("((*)(*))((*");
    //let s = String::from("(*))");
    println!("{}", s);
    println!(" --> {}", SolutionValidParens::check_valid_string(s));

    let mut v = Vec::new();
    v.push(vec!['1','1','0','0','0']);
    v.push(vec!['1','1','0','0','0']);
    v.push(vec!['0','0','1','0','0']);
    v.push(vec!['0','0','0','1','1']);
    println!("{:?}", v);
    println!(" --> {}", SolutionNumIslands::num_islands(v));

    let mut grid = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
    println!("{:?}", grid);
    println!(" --> {}", SolutionMinPath::min_path_sum(grid));
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
        let mut running_count = 0;
        let mut num_stars_available = 0;
        // Let's go forward
        for c in s.chars() {
            match c {
                '(' => {
                    running_count += 1;
                }
                ')' => {
                    running_count -= 1;
                }
                '*' => {
                    num_stars_available += 1;
                }
                _   => { }
            }
            if running_count < 0 {
                if num_stars_available > 0 {
                    num_stars_available -= 1;
                    running_count += 1;
                }
                else {
                    return false;
                }
            }
        }

        // Let's go backward
        running_count = 0;
        num_stars_available = 0;
        for c in s.chars().rev() {
            match c {
                '(' => {
                    running_count -= 1;
                }
                ')' => {
                    running_count += 1;
                }
                '*' => {
                    num_stars_available += 1;
                }
                _   => { }
            }
            if running_count < 0 {
                if num_stars_available > 0 {
                    num_stars_available -= 1;
                    running_count += 1;
                }
                else {
                    return false;
                }
            }
        }
        return true;
    }
}

struct SolutionNumIslands { }

impl SolutionNumIslands {
    fn withinOne(pt: (i32, i32), points: &Vec<(i32, i32)>) -> bool {
        for (x, y) in points.iter() {
            if (pt.0 + 1 == *x && pt.1 == *y) || (pt.0 - 1 == *x && pt.1 == *y) {
                return true;
            }
            if (pt.0 == *x && pt.1 + 1 == *y) || (pt.0 == *x && pt.1 - 1 == *y) {
                return true;
            }
        }
        return false;
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islands: HashSet<Vec<(i32, i32)>> = HashSet::new();
        let n_rows = grid.len();
        if n_rows == 0 {
            return 0;
        }
        let n_cols = grid[0].len();
        if n_cols == 0 {
            return 0;
        }


        for i in (0..n_rows) {
            for j in (0..n_cols) {
                let i_32 = i as i32;
                let j_32 = j as i32;
                if grid[i][j] == '1' {
                    println!("Checking ({}, {})", i_32, j_32);
                    // Check if adjacent to existing island(s)
                    let mut adjacent_islands:  HashSet<Vec<(i32, i32)>> = islands.clone();
                    adjacent_islands.retain(|cur_island| {
                        SolutionNumIslands::withinOne((i_32, j_32 as i32), cur_island)
                    });
                    islands.retain(|cur_island| {
                        ! SolutionNumIslands::withinOne((i_32, j_32 as i32), cur_island)
                    });

                    println!("  Adjacent islands: {:?}", adjacent_islands);
                    println!("           Islands: {:?}", islands);

                    if adjacent_islands.len() > 0 {
                        let mut new_island: Vec<(i32, i32)> = Vec::new();
                        new_island.push((i_32, j_32));
                        for cur_island in adjacent_islands.iter() {
                            for pt in cur_island.into_iter() {
                                new_island.push(*pt);
                            }
                        }
                        islands.insert(new_island);
                    }
                    // A brand new island!
                    else {
                        islands.insert(vec![(i_32, j_32)]);
                    }
                    println!("     Final islands: {:?}", islands);
                }
            }
        }
        return islands.len() as i32;
    }
}

struct SolutionMinPath { }

impl SolutionMinPath {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n_i = grid.len();
        if n_i == 0 {
            return 0;
        }
        let n_j = grid[0].len();
        if n_j == 0 {
            return 0;
        }
        let mut result = vec![vec![-1; n_j]; n_i];
        result[0][0] = grid[0][0];
        for j in 1..n_j {
            result[0][j] = result[0][j-1] + grid[0][j];
        }
        for i in 1..n_i {
            result[i][0] = result[i-1][0] + grid[i][0];
        }
        for i in 1..n_i {
            for j in 1..n_j {
                let m = min(result[i-1][j], result[i][j-1]);
                result[i][j] = m + grid[i][j];
            }
        }
        return result[n_i-1][n_j-1];
    }
}