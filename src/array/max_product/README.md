## O(n)

---

어려운 경우는 <br>

- 음수를 만났을 때와
- 0을 만났을 때이다.

음수를 만나면 대소 관계가 뒤집히기 때문에 처리하기가 어려운데 이 때문에 <br>
해답의 코드에서는 현재 인덱스까지의 최대 곱과 최소 곱을 모두 계산한다. <br>
계속 기록해나가는 최대 곱과 최소 곱을 계산할 때는 현재 인덱스 값이 반드시 <br>
포함된 값을 계산한다. 이렇게 계산함으로써 다음 연속된 부분 배열의 최대 곱을 <br>
효율적으로 계산해나갈 수 있다.
