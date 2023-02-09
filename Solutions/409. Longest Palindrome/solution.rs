/*

409. Longest Palindrome

Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.

Letters are case sensitive, for example, "Aa" is not considered a palindrome here.

Example 1:
    Input: s = "abccccdd"
    Output: 7
    Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.


Example 2:
    Input: s = "a"
    Output: 1
    Explanation: The longest palindrome that can be built is "a", whose length is 1.


Constraints:
    1 <= s.length <= 2000
    s consists of lowercase and/or uppercase English letters only.

*/

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = vec![0; 256];
        let mut counter = 0;
        for b in s.bytes() {
            let ascii = b as u8;
            map[ascii as usize] += 1;
            if map[ascii as usize] % 2 == 0 {
                counter += 1;
            }
        }
        counter = counter * 2;
        if counter < s.len() as i32 {
            counter += 1;
        }
        return counter;
    }
}
