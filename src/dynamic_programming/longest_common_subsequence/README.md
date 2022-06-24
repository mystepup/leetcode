## O(nm)

---

text1과 text2의 i, j번째 인덱스까지의 LCS를 dp[i][j]라고하면 dp[i + 1][j + 1]의 값은  
i, j에서의 text1, text2값이 같다면 dp[i][j] + 1이 되고 아니라면 max(dp[i][j+1], dp[i+1][j])가  
된다. 하지만 이렇게 계산했을 경우 최대 길이가 넘어가게 되므로 dp를 text1.len() + 1 x text2.len() + 1  
인 2차원배열로 만들어준다. 그리고 제일 바닥 값들을 0으로 셋팅해줘서 text1, text2의 0, 0번째 값이 dp[1][1]에 오게 한다.
