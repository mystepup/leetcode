??

---

s를 처음부터 순회하면서 wordDict의 단어를 하나씩 비교하는데 비교하는 방법은  
wordDict를 s의 현재 index에서 뺐을 때 s(s`)가 wordDict로 이루어져 있는지 확인한다. 만약 wordDict로 이루어져 있다면 s`에서 현재 index까지의 s가 wordDict의 word인지 체크한다.

이 때 memoization으로 s의 index마다 wordbreak결과를 기록한 array를 사용하며 중복 계산을  
방지한다.

예를 들어 s가 abcde, wordDict가 [a, ab, abc, abcd, abcde]라면??

    dp가 없다면 i가 1일 때 dp[1]이 true임을 알고 있음에도 불구하고 i가 2인 경우에 wordDict의 a에 대해서 계산할 때 dp[1]을 다시 계산하게 된다.
