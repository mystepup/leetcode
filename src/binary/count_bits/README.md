## O(nlong), O(logn)

---

nlogn방식은 hammingWeight를 n번하는 것이다.

logn으로 풀이할수도 있는데  
x가 짝수이면 LSB는 0이고 x를 2로 나눈 수와 x의 hammingweight는 같다.  
x가 홀수이면 LSB는 1이고 x를 2로 나눈 수와 x의 hammingweight는 x가 1이 크다.
