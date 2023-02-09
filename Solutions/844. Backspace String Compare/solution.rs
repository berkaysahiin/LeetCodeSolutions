/*

844. Backspace String Compare

Example 1:
    Input: s = "ab#c", t = "ad#c"
    Output: true
    Explanation: Both s and t become "ac".

Example 2:
    Input: s = "ab##", t = "c#d#"
    Output: true
    Explanation: Both s and t become "".

Example 3:
    Input: s = "a#c", t = "b"
    Output: false
    Explanation: s becomes "c" while t becomes "b".

    Constraints:


1 <= s.length, t.length <= 200
s and t only contain lowercase letters and '#' characters.

*/

use std::collections::LinkedList;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
      let mut s_ll: LinkedList<char> = LinkedList::new();
      let mut t_ll: LinkedList<char> = LinkedList::new();

      for c in s.chars() {
        if c == '#' {
          s_ll.pop_back();
          continue;
        }

        s_ll.push_back(c);
      }

      for c in t.chars() {
        if c == '#' {
          t_ll.pop_back();
          continue;
        }

        t_ll.push_back(c);
      }

      return s_ll == t_ll;
    }
}
