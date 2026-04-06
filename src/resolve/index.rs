use std::collections::HashMap;

use crate::model::document::Document;
use crate::model::objects::Child;

/// Index of reusable component ID → Child node.
pub struct NodeIndex {
    nodes: HashMap<String, Child>,
}

impl NodeIndex {
    pub fn build(doc: &Document) -> Self {
        let mut nodes = HashMap::new();
        for child in &doc.children {
            index_recursive(child, &mut nodes);
        }
        NodeIndex { nodes }
    }

    pub fn get(&self, id: &str) -> Option<&Child> {
        self.nodes.get(id)
    }
}

fn index_recursive(child: &Child, nodes: &mut HashMap<String, Child>) {
    if child.entity().reusable == Some(true) {
        nodes.insert(child.id().to_string(), child.clone());
    }
    if let Some(children) = child.children() {
        for c in children {
            index_recursive(c, nodes);
        }
    }
}
