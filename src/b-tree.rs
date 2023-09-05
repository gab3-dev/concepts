#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            root: None,
        }
        
    }
    
    fn insert(&mut self, value: i64) {
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            }
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }
    
    fn insert_recursive(node: &mut Box<Node>, value: i64) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        }
    }
}

impl Node {
    fn new(value: i64) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(n: Node) -> Self {
        Some(Box::new(n))
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(5);
    tree.insert(3);
    tree.insert(4);
    tree.insert(6);
}
