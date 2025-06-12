// # 3423. Maximum Difference Between Adjacent Elements in a Circular Array

// Given a circular array nums, find the maximum absolute difference between adjacent elements.

// Note: In a circular array, the first and last elements are adjacent.

// Example 1:

// Input: nums = [1,2,4]

// Output: 3

// Explanation:

// Because nums is circular, nums[0] and nums[2] are adjacent. They have the maximum absolute difference of |4 - 1| = 3.

// Example 2:

// Input: nums = [-5,-10,-5]

// Output: 5

// Explanation:

// The adjacent elements nums[0] and nums[1] have the maximum absolute difference of |-5 - (-10)| = 5.

// Constraints:

//     2 <= nums.length <= 100
//     -100 <= nums[i] <= 100

// Time comp: O(n), 0ms


struct Solution {

}

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        nums.iter()
            // zip cuts cycle to same len as nums
            .zip(nums.iter().cycle().skip(1))
            .map(|(n1, n2)| n1.abs_diff(*n2))
            .max()
            .unwrap_or(0) as i32
    }
}   

fn main() {
    let answer = Solution::max_adjacent_distance(vec![-2,1,-5]);
    println!("{answer:?}");
}