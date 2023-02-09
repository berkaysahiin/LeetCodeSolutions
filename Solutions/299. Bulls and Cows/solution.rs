/*
 
299. Bulls and Cows

You are playing the Bulls and Cows game with your friend.

You write down a secret number and ask your friend to guess what the number is. When your friend makes a guess, you provide a hint with the following info:

The number of "bulls", which are digits in the guess that are in the correct position.
The number of "cows", which are digits in the guess that are in your secret number but are located in the wrong position. Specifically, the non-bull digits in the guess that could be rearranged such that they become bulls.
Given the secret number secret and your friend's guess guess, return the hint for your friend's guess.

The hint should be formatted as "xAyB", where x is the number of bulls and y is the number of cows. Note that both secret and guess may contain duplicate digits.

Example 1:
    Input: secret = "1807", guess = "7810"
    Output: "1A3B"


Example 2:
    Input: secret = "1123", guess = "0111"
    Output: "1A1B"

Constraints:
    1 <= secret.length, guess.length <= 1000
    secret.length == guess.length
    secret and guess consist of digits only.
*/

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;

        let mut secret_chars: Vec<u8> = vec![0; 256];
        let mut guess_chars: Vec<u8> = vec![0; 256];

        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                bulls += 1;
                continue;
            }

            secret_chars[s as usize] += 1;
            guess_chars[g as usize] += 1;
        }

        for g in guess.chars() {
            if secret_chars[g as usize] > 0 && guess_chars[g as usize] > 0 {
                cows += 1;
                guess_chars[g as usize] -= 1;
                secret_chars[g as usize] -= 1;
            }
        }
 
        format!("{}A{}B", bulls, cows)
    }
}
