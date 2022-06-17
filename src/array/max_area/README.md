## O(n)

---

    두 막대기가 이루는 넓이가 가장 넓게 해야함

    1. 넓이가 가장 넓은 맨 처음, 마지막 막대기를 기둥으로 삼는 컨테이너로 시작한다.
    2. 이 두 막대기 사이에 있는 더 긴 막대기에 의해 더 큰 너비를 가지는 컨테이너가
        생길수도 있다.
    3. 두 사이드 기둥중 더 작은 기둥을 안쪽으로 하나씩 옮기면서 더 큰 넓이가
        존재하는지 확인한다.

    [1, 8] // 1~2: 1
    [1, 8, 6] // 2~3: 6
    [1, 8, 6, 2] // 6
    [1, 8, 6, 2, 5] // 2~5: 15
    [1, 8, 6, 2, 5, 4] // 2~6: 16
    [1, 8, 6, 2, 5, 4, 8] // 2~7: 35
    [1, 8, 6, 2, 5, 4, 8, 3] // 35
    [1, 8, 6, 2, 5, 4, 8, 3, 7] // 2~9: 49