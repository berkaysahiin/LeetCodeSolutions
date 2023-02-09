/*

Given two strings s and t, determine if they are isomorphic.

Two strings s and t are isomorphic if the characters in s can be replaced to get t.

All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.


Example 1:

Input: s = "egg", t = "add"
Output: true
Example 2:

Input: s = "foo", t = "bar"
Output: false
Example 3:

Input: s = "paper", t = "title"
Output: true
 

Constraints:

1 <= s.length <= 5 * 104
t.length == s.length
s and t consist of any valid ascii character.

*/

use std::collections::HashMap;

impl Solution {
   pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map_s: HashMap<char, char> = HashMap::new();
    let mut map_t: HashMap<char, char> = HashMap::new();

    for (c1, c2) in s.chars().zip(t.chars()) {
        if map_s.contains_key(&c1) && map_s.get(&c1).unwrap() !=  &c2 { 
            return false;
        }

        if map_t.contains_key(&c2) && map_t.get(&c2).unwrap() !=  &c1 { 
            return false;
        }

        map_s.insert(c1,c2);
        map_t.insert(c2,c1);
    }

    true
    }
}
