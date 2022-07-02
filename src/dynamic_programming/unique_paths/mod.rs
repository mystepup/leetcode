use super::DynamicProgramming;

impl DynamicProgramming {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp:Vec<Vec<i32>> = vec![vec![0;n];m];
        
        for i in 0..m {
            dp[i][0] = 1;
        }

        for j in 0..n {
            dp[0][j] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}