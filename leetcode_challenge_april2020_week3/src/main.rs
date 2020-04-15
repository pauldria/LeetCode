fn main() {
    let v = vec![5,4];
    println!("{:?}", v);
    println!(" --> {:?}", SolutionProductExceptSelf::product_except_self(v));

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