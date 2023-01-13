use std::env::args;
pub struct Recursion {}

fn rec(nums: &Vec<i32>, output: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, idx: usize) {
    output.push((*subset).to_vec());
    let size = nums.len();
    for i in idx..size {
        let x: i32 = nums[i];
        subset.push(x);
        rec(nums, output, subset, i + 1);
        subset.pop();
    }
}

impl Recursion {
    
    pub fn power_set(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();
        let mut subset: Vec<i32> = Vec::new();
        rec(&nums, &mut output, &mut subset, 0);
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in &output {
            res.push((*i).to_vec());
        }
        return res;
    }
}

fn main() {
    let mut args: Vec<String> = args().collect();
    let mut set: Vec<i32> = Vec::new();

    for i in &mut args[1..] {
        set.push(i.parse::<i32>().unwrap())
    }

    println!("{:?}", Recursion::power_set(set));
}