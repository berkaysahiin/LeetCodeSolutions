/*
977. Squares of a Sorted Array

Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

Example 1:
Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
Explanation: After squaring, the array becomes [16,1,0,9,100].
After sorting, it becomes [0,1,9,16,100].

Example 2:
Input: nums = [-7,-3,2,3,11]
Output: [4,9,9,49,121]

Constraints:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums is sorted in non-decreasing order.

*/

use std::collections;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
      let mut negatives = vec![];
      let mut sorted_sqr = vec![];
      let mut i: usize = 0;

      while i < nums.len() {
        let value = nums[i];
        let value_sqr = value * value;

        if value < 0 {
          negatives.push(value_sqr);
          i += 1;
          continue;
        }

        if negatives.is_empty() {
          sorted_sqr.push(value_sqr);
          i += 1;
          continue;
        }

        let smallest_negative_sqr = negatives[negatives.len() - 1];

        if value_sqr < smallest_negative_sqr {
          sorted_sqr.push(value_sqr);
          i += 1;
        }
        else {
          sorted_sqr.push(smallest_negative_sqr);
          negatives.pop();
        }
      }

      negatives.reverse();
      for &num in negatives.iter() {
        sorted_sqr.push(num);
      }

      sorted_sqr
    }
}

