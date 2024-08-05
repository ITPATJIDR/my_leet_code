struct Solution;

pub impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }
}


fn main() {
    let test = Solution::can_be_equal(target, arr);
}
