struct Node<'a> {
    name: String,
    neighbors: (&'a Node<'a>, &'a Node<'a>),
}

struct Graph<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: String, left: String, right: String) {
        let left = self.nodes.iter().find(|n| n.name == left).unwrap();
        let right = self.nodes.iter().find(|n| n.name == right).unwrap();

        self.nodes.push(Node {
            name: node,
            neighbors: (left, right),
        });
    }
}
