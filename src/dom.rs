use std::collections::HashMap;

pub struct Arena<T> {
    next_id: i64,
    node_list: HashMap<i64, Node<T>>
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            next_id: 0,
            node_list: HashMap::new()
        }
    }

    pub fn node(&mut self, data: T) -> &mut Node<T> {
        let id = self.next_id;
        let node = Node {
            id,
            parent: None,
            children: vec![],
            data
        };
        self.next_id += 1;
        self.node_list.insert(id, node);
        self.node_list.get_mut(&id).unwrap()
    }

    pub fn find(&mut self, id: &i64) -> Option<&mut Node<T>> {
        self.node_list.get_mut(id)
    }

    pub fn detach(&mut self, child_id: &i64) {
        let Some(prev_parent_id) = child.parent else { return };
        let Some(prev_parent) = self.find(&prev_parent_id) else { return };
        let Some(pos) = prev_parent.children.iter().position(|x| *x == child.id) else { return };
        prev_parent.children.remove(pos);
    }

    pub fn append(&mut self, parent: &mut Node<T>, child: &mut Node<T>) {
        self.detach(child);
        child.parent = Some(parent.id);
        parent.children.push(child.id);
    }
}

pub struct Node<T> {
    id: i64,
    parent: Option<i64>,
    children: Vec<i64>,
    data: T
}

