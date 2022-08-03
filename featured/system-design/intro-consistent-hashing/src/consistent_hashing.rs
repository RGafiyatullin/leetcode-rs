use std::collections::{HashMap, HashSet};

pub struct ConsistentHashing {
    next_unused_node_id: i32,
    node_to_keys: HashMap<i32, HashSet<i32>>,
    key_to_nodes: HashMap<i32, HashSet<i32>>,
    node_balancing: (Vec<i32>, usize),
}

impl ConsistentHashing {
    pub fn new(initial_nodes: i32) -> Self {
        Self {
            next_unused_node_id: initial_nodes + 1,
            node_to_keys: (1..=initial_nodes)
                .map(|node_id| (node_id, Default::default()))
                .collect(),
            key_to_nodes: Default::default(),
            node_balancing: ((1..=initial_nodes).collect(), initial_nodes as usize - 1),
        }
    }

    pub fn get_node_for_key(&mut self, key: i32) -> i32 {
        let key_to_nodes = &mut self.key_to_nodes;
        let node_to_keys = &mut self.node_to_keys;
        let node_balancing = &mut self.node_balancing;

        key_to_nodes
            .entry(key)
            .or_insert_with(move || {
                node_balancing.1 += 1;
                let node_id = node_balancing.0[node_balancing.1 % node_balancing.0.len()];
                assert!(node_to_keys
                    .get_mut(&node_id)
                    .expect("Balancing returned a non-existent node")
                    .insert(key));
                [node_id].into()
            })
            .iter()
            .copied()
            .next()
            .expect("There must be at least node there")
    }

    pub fn remove_node(&mut self, gone_node_id: i32) -> i32 {
        let node_to_keys = &mut self.node_to_keys;
        let node_balancing = &mut self.node_balancing;

        let gone_node_keyset = node_to_keys
            .remove(&gone_node_id)
            .expect("There must have been such a node");

        let (&heir_node_id, heir_keyset) = node_to_keys
            .iter_mut()
            .min_by_key(|(_k, v)| v.len())
            .expect("addNode will be called only if there is at least one node in the cluster.");
        heir_keyset.extend(gone_node_keyset);

        let drained = node_balancing
            .0
            .drain(..)
            .filter(|id| *id != gone_node_id)
            .collect();
        node_balancing.0 = drained;

        heir_node_id
    }

    pub fn add_node(&mut self) -> Vec<i32> {
        let added_node_id = self.next_unused_node_id;
        self.next_unused_node_id += 1;

        let node_balancing = &mut self.node_balancing;
        let node_to_keys = &mut self.node_to_keys;
        let key_to_nodes = &mut self.key_to_nodes;

        let (&companion_node_id, companion_keyset) = node_to_keys
            .iter()
            .min_by_key(|(_k, v)| v.len())
            .expect("addNode will be called only if there is at least one node in the cluster.");
        let companion_keyset = companion_keyset.to_owned();

        node_to_keys.insert(added_node_id, companion_keyset.to_owned());
        companion_keyset.into_iter().for_each(|key| {
            key_to_nodes.entry(key).or_default().insert(added_node_id);
        });

        node_balancing.0.push(added_node_id);

        vec![added_node_id, companion_node_id]
    }

    pub fn get_keys_in_node(&self, node_id: i32) -> Vec<i32> {
        // eprintln!("self.nodes: {:#?}", self.nodes);
        self.node_to_keys
            .get(&node_id)
            .into_iter()
            .flat_map(|v| v.iter().copied())
            .collect()
    }
}
