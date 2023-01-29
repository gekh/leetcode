use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

struct Node {
    frequency: i32,
    call_counter: u32,
    value: i32,
}

pub struct LFUCache {
    capacity: usize,
    call_counter: u32,
    nodes: BTreeMap<i32, Node>,
    hash: BTreeSet<(i32,u32,i32)>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            call_counter: 1,
            nodes: BTreeMap::new(),
            hash: BTreeSet::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {

        self.call_counter += 1;
        if let Entry::Occupied(mut node) = self.nodes.entry(key) {
            let mut node_mut = node.get_mut();

            self.hash.take(&(node_mut.frequency, node_mut.call_counter, key));

            node_mut.frequency += 1;
            node_mut.call_counter = self.call_counter;

            self.hash.insert((node_mut.frequency, node_mut.call_counter, key));

            return node.get().value;
        }

        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.call_counter += 1;

        if let Entry::Occupied(mut node) = self.nodes.entry(key) {
            let mut node_mut = node.get_mut();

            self.hash.take(&(node_mut.frequency, node_mut.call_counter, key));

            node_mut.value = value;
            node_mut.call_counter = self.call_counter;
            node_mut.frequency += 1;

            self.hash.insert((node_mut.frequency, node_mut.call_counter, key));
        } else if self.capacity > 0 {
            let new_node = Node {
                value,
                frequency: 1,
                call_counter: self.call_counter,
            };

            if self.nodes.len() >= self.capacity {
                // let d = self.hash.pop_first().unwrap();
                let d = self.hash.iter().next().unwrap().clone();
                self.hash.remove(&d);
                self.nodes.remove(&d.2);
            }

            self.hash.insert((new_node.frequency, new_node.call_counter, key));
            self.nodes.insert(key, new_node);
        }
    }
}
