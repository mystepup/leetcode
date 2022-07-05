{
  //Definition for Node.
  class Node {
    val: number;
    neighbors: Node[];
    constructor(val?: number, neighbors?: Node[]) {
      this.val = val === undefined ? 0 : val;
      this.neighbors = neighbors === undefined ? [] : neighbors;
    }
  }

  function dfs(node: Node, copy: Node, visited: Node[]) {
    visited[copy.val] = copy;

    for (const n of node.neighbors) {
      if (visited[n.val] == null) {
        const newNode = new Node(n.val);
        copy.neighbors.push(newNode);
        dfs(n, newNode, visited);
      } else {
        copy.neighbors.push(visited[n.val]);
      }
    }
  }

  function cloneGraph(node: Node | null): Node | null {
    if (node == null) return null;
    const copy: Node = new Node(node.val);
    const visited: Node[] = new Array(101);
    visited.fill(null);

    dfs(node, copy, visited);
    return copy;
  }
}
