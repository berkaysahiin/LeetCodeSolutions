/*

Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.

 

Example 1:

Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4
Example 2:

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1
 

Constraints:

1 <= nums.length <= 104
-104 < nums[i], target < 104
All the integers in nums are unique.
nums is sorted in ascending order.

*/

fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mid_index = nums.len() / 2;
    let mid = &nums[mid_index];

    if *mid == target {
        return mid_index as i32;
    }

    if *mid > target {
        return search((&nums[..mid_index]).to_vec(), target);
    }

    let result = search((&nums[mid_index + 1..]).to_vec(), target);

    if result == -1 {
        return -1;
    }
    else {
        return result + mid_index as i32 + 1;
    }
}
