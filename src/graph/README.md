## DFS Depth-first search

```
procedure DFS(G, v)_recursive is
    label v as discovered
    for all directed edges from v to w that are in G.adjacentEdges(v) do
        if vertex w is not labeled as discovered then
            recursively call DFS(G, w)

```

```
procedure DFS_iterative(G, v) is
    let S be a stack
    S.push(v)
    while S is not empty do
        v = S.pop()
        if v is not labeled as discovered then
            label v as discovered
            for all edges from v to w in G.adjacentEdges(v) do
                S.push(w)
```

```
procedure DFS_iterative(G, v)_iterator is
    let S be a stack
    S.push(iterator of G.adjacentEdges(v))
    while S is not empty do
        if S.peek().hasNext() then
            w = S.peek().hasNext()
            if w is not labeled as discovered then
                label w as discovered
                S.push(iterator of G.adjacentEdges(w))
        else
            S.pop()
```

## BFS Breadth-first Search

```
proceduer BFS(G, root) is
    let Q be a queue
    label root as visited
    Q.enqueue(root)
    while Q is not empty do
        v := Q.dequeue()
        for all edges from v to w in G.adjacentEdges(v) do
            if w is not labeled as visited then
                label w as visited
                Q.enqueue(w)
```

1. BFS는 DFS stack iterative버전에서 stack을 queue로 바꾼거랑 비슷한 방식으로 짜여진다.

2. BFS는 Queue에 넣기전에 방문표시를 하지만 DFS는 Stack에서 pop을 한 후에 방문 마크를 남긴다. (= pop을 한 뒤에 방문한 표시가 없다면 방문 표시를 한 후에 인접 노드 검사, queue를 하기전에 방문여부를 검사하고 첫 방문만 Queue에 삽입)
