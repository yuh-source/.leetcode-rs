// # 4. Median of Two Sorted Arrays

// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

// The overall run time complexity should be O(log (m+n)).

// Example 1:

// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.

// Example 2:

// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

// Constraints:

//     nums1.length == m
//     nums2.length == n
//     0 <= m <= 1000
//     0 <= n <= 1000
//     1 <= m + n <= 2000
//     -106 <= nums1[i], nums2[i] <= 106

// Approach 3 from: https://leetcode.com/problems/median-of-two-sorted-arrays/editorial/
// reading even just the first partition seperation part gives a lot of insights


struct Solution {

}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // we always want to binary search smaller array
        let (small, large) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let mut low = 0;
        let mut high = small.len();

        while low <= high {
            // the left halves of both arrays is always (m + n + 1) / 2
            // if partition B ( Bleft and Bright) is equal to (m + n + 1) / 2 - partitionA
            // diagrams in editorial
            let small_split = (low + high) / 2;
            let large_split = (small.len() + large.len() + 1) / 2 - small_split;

            // the max of the left split is adjacent to the min of the right split
            let small_max = if small_split == 0 { i32::MIN } else { small[small_split - 1] };
            let small_min = if small_split == small.len() { i32::MAX } else { small[small_split] };

            let large_max = if large_split == 0 { i32::MIN } else { large[large_split - 1] };
            let large_min = if large_split == large.len() { i32::MAX } else { large[large_split] };


            // the min values are from both right splits therefore, 
            // if they are lesser than or equal to the max of the left splits
            // then the arrays are partitioned at the midpoint
            if small_max <= large_min && large_max <= small_min {
                if (small.len() + large.len()) % 2 == 0 {
                    return (small_max.max(large_max) + small_min.min(large_min)) as f64 / 2.0;
                } else {
                    return (small_max.max(large_max)) as f64;
                }
            // the small max being greater than the left min
            // implies that the small max is too large to be in the smaller half
            // and the high value should be decreased, moving small max into the right half
            } else if small_max > large_min {
                high = small_split - 1;
            // or large min is too small to be in the right split
            // and is moved to the left split
            } else {
                low = small_split + 1;
            }
            // more diagrams on this in the editorial
        }

        panic!();
    }
}

fn main() {
    let answer = Solution::find_median_sorted_arrays(vec![1,2,3,4,5,14], vec![6,7,8,9,10,11,12,13]);
    println!("{answer:?}");
}