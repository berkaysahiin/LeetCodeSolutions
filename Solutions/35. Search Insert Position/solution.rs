/* 35. Search Insert Position

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Example 1:
    Input: nums = [1,3,5,6], target = 5
    Output: 2

Example 2:
    Input: nums = [1,3,5,6], target = 2
    Output: 1

Example 3:
    Input: nums = [1,3,5,6], target = 7
    Output: 4

Constraints:

1 <= nums.length <= 104
-104 <= nums[i] <= 104
nums contains distinct values sorted in ascending order.
-104 <= target <= 104

*/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
      let size: i32 = nums.len() as i32;
      let mut start_index: i32 = 0;
      let mut end_index: i32 = size - 1;

      while start_index <= end_index {
        let mid_index: i32 = start_index + (end_index - start_index) / 2;
        let mid_value = nums[mid_index as usize];

        if mid_value == target {
          return mid_index;
        }

        if mid_value < target {
          start_index = mid_index + 1;
          continue;
        }

        end_index = mid_index - 1;
      }

      start_index
    }
}
