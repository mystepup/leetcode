mod graph;
use graph::Graph;

fn main() {
    let result = Graph::can_finish(3, vec![vec![2, 0], vec![0,1]]);

    println!("{:?}", result);
}
