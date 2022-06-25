use super::DynamicProgramming;

use std::collections::HashSet;

impl DynamicProgramming {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp: Vec<bool> = vec![false; s.len() + 1];
        dp[0] = true;

        let dict: HashSet<String> = word_dict.into_iter().collect();

        for i in 0..=s.len() {
            for word in dict.iter() {
                let n = word.len();
                
                if i >= n {
                    if dp[i - n] && (&s[(i - n)..i]).eq(&word[..]) {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }

        dp[dp.len() - 1]
    }
}