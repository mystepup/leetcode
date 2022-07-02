use super::DynamicProgramming;

impl DynamicProgramming {
    pub fn num_decodings(s: String) -> i32 {
        let chars = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];

        dp[0] = 1;
        dp[1] = if chars[0] == b'0' { 0 } else { 1 };

        for i in 2..dp.len() {
            if chars[i - 1] != b'0' {
                dp[i] = dp[i - 1];
            }

            let n = (chars[i - 2] - b'0') * 10 + (chars[i - 1] - b'0');

            if n >= 10 && n <= 26 {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len()]
    }
}

// 1                            1    
// 1, 2                         1, 2, | 12
// 1, 2, 3                      1, 2, 3 | 1, 23 | 12, 3
// 1, 2, 3, 1                   1, 2, 3, 1 | 1, 23, 1 | 12, 3, 1
// 1, 2, 3, 1, 1                1, 2, 3, 1, 1 | 1, 2, 3, 11 | 1, 23, 1, 1 | 1, 23, 11 | 12, 3, 1, 1 | 12, 3, 11
// 1, 2, 3, 1, 1, 0             1, 2, 3, 1, 10 | 1, 23, 1, 10 | 12, 3, 1, 10 # 0을 포함하는 건 leading zero밖에 없음