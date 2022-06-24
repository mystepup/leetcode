use crate::dynamic_programming::DynamicProgramming;

impl DynamicProgramming {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

        let mut dp = vec![vec![0;text2.len() + 1];text1.len() + 1];

        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                dp[i + 1][j + 1] = if c1 == c2 {
                    dp[i][j] + 1
                } else {
                    std::cmp::max(dp[i][j + 1], dp[i + 1][j])
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}