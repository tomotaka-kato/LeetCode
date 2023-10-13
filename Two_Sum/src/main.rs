impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let chance: Vec<i32> = nums.iter().cloned().filter(|&x| x < target).collect();
        chance
    }
}

fn main() {
    println!("Hello, world!");
}
