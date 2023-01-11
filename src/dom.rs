use std::collections::HashMap;
use std::fmt;


struct Node {
    children: Vec<Node>,
    node_type:NodeType
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_name: String,
    attributes: HashMap<String, String>
}

impl Node {
    fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}
impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t)
                NodeType::Element(ref e) => write(f, "{:?}", e)
            }
        }
    }
}