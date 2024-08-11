pub enum NodeType {
    DocumentNode
}

#[allow(dead_code)]
pub trait Node {
    fn node_type(&self) -> NodeType;
    fn node_name(&self) -> String;
    fn owner_document(&self) -> Option<impl Document>;
    fn base_uri(&self) -> String;
    fn is_connected(&self) -> bool;
    fn parent_node(&self) -> Option<impl Node>;
    fn first_child(&self) -> Option<impl Node>;
    fn next_sibling(&self) -> Option<impl Node>;
    fn append_child<T>(&self, child: _Node<T>);
}

#[allow(dead_code)]
pub trait Document : Node {
    fn base_url(&self) -> String;
}

pub type _Node<T> = rctree::Node<T>;

pub struct DocumentData {
    url: String,
}
pub type DocumentNode = _Node<DocumentData>;

impl Document for DocumentNode {
    fn base_url<'a>(&'a self) -> String {
        self.borrow().url.clone()
    }
}

impl Node for DocumentNode {
    fn node_type(&self) -> NodeType {
        NodeType::DocumentNode
    }

    fn node_name(&self) -> String {
        String::from("#document")
    }

    fn owner_document(&self) -> Option<impl Document> {
        Some(self.clone())
    }

    fn base_uri(&self) -> String {
        self.base_url()
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn parent_node(&self) -> Option<impl Node> {
        self.parent()
    }

    fn first_child(&self) -> Option<impl Node> {
        self.first_child()    
    }

    fn next_sibling(&self) -> Option<impl Node> {
        self.next_sibling()    
    }

    fn append_child<T>(&self, child: _Node<T>) {
        self.append(child);
    }
}
