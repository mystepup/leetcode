adjacent graph를 만들어서 dfs를 할껀데 모든 점을 방문할 수 있어야하고  
수강과목 두개가 의존성이 생기면 안된다(서로를 가리키면 안됨)

visited는 현재 노드에서 cycle이 있는지 없는지 검사하므로 현재 노드에서의 검사가 끝나면 false로  
두는데, checked는 이전 노드들에서 부가적으로 검사된 후손 노드들이 다음 노드에서의 cycle검사 때
다시 검사가 되지 않게 하기 위해 한번 검사된 노드들을 기록하는 배열이다.

DAG그래프의 cycle이 존재하느냐 아니냐로도 볼수 있음
