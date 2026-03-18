use std::{any::Any, cell::RefCell, collections::HashMap, hash::Hasher, rc::Rc};

use crate::rng::RNG;

struct Node {
    best_cost: u32,
    unique_part: Rc<NodeUniquePart>,
    visited: bool, // todo handle arbitrary data for nodes where such a thing matters
}

struct NodeUniquePart {
    layer_id: &'static str,
    rng: Option<RNG>,
    custom_data: Box<dyn DynCustomData>,
}

impl Eq for NodeUniquePart {}

impl PartialEq for NodeUniquePart {
    fn eq(&self, other: &Self) -> bool {
        self.layer_id == other.layer_id
            && self.rng == other.rng
            && self.custom_data.as_ref().equal(other.custom_data.as_ref())
    }
}

impl std::hash::Hash for NodeUniquePart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.layer_id.hash(state);
        self.rng.hash(state);
        self.custom_data.dyn_hash(state);
    }
}
trait DynCustomData: DynsafeEq + DynsafeHash + Any {}
impl<T: ValidCustomData + 'static> DynCustomData for T {}
trait DynsafeHash {
    fn dyn_hash(&self, state: &mut dyn Hasher);
}
impl<T: std::hash::Hash> DynsafeHash for T {
    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state);
    }
}

trait DynsafeEq {
    fn as_any(&self) -> &dyn Any;
    fn equal(&self, other: &dyn DynsafeEq) -> bool;
}

impl<T: PartialEq + 'static> DynsafeEq for T {
    fn equal(&self, other: &dyn DynsafeEq) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |other| self == other)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ValidCustomData: std::hash::Hash + Eq + Clone {}
impl<T: std::hash::Hash + Eq + Clone> ValidCustomData for T {}
#[derive(Clone, Copy)]
pub struct NodeHandle(usize);

#[derive(Default)]
pub struct NodeHeap {
    nodes: RefCell<Vec<Node>>,
    index: RefCell<HashMap<Rc<NodeUniquePart>, NodeHandle>>,
}
impl NodeHeap {
    pub fn get_or_construct_node(&self, layer_id: &'static str, rng: Option<RNG>) -> NodeHandle {
        let unique_part = Rc::new(NodeUniquePart {
            layer_id,
            rng,
            custom_data: Box::new(()),
        });
        {
            if let Some(id) = self.index.borrow().get(&unique_part) {
                return *id;
            };
        }
        let node = Node {
            best_cost: u32::MAX,
            unique_part: unique_part.clone(),
            visited: false,
        };

        let mut nodes = self.nodes.borrow_mut();
        let id = NodeHandle(nodes.len());
        nodes.push(node);
        self.index.borrow_mut().insert(unique_part, id);
        id
        // first try to find a matching node
    }

    pub fn get_or_construct_node_with_custom_data<T: ValidCustomData + 'static>(
        &self,
        layer_id: &'static str,
        rng: Option<RNG>,
        custom_data: T,
    ) -> NodeHandle {
        let unique_part = Rc::new(NodeUniquePart {
            layer_id,
            rng,
            custom_data: Box::new(custom_data),
        });
        {
            if let Some(id) = self.index.borrow().get(&unique_part) {
                return *id;
            };
        }
        let node = Node {
            best_cost: u32::MAX,
            unique_part: unique_part.clone(),
            visited: false,
        };

        let mut nodes = self.nodes.borrow_mut();
        let id = NodeHandle(nodes.len());
        nodes.push(node);
        self.index.borrow_mut().insert(unique_part, id);
        id
        // first try to find a matching node
    }

    pub fn mark_visited(&self, node: NodeHandle) {
        self.nodes.borrow_mut()[node.0].visited = true
    }

    pub fn best_cost(&self, node: NodeHandle) -> u32 {
        self.nodes.borrow()[node.0].best_cost
    }
    pub fn update_best_cost(&self, node: NodeHandle, new_best: u32) {
        self.nodes.borrow_mut()[node.0].best_cost = new_best;
    }

    pub fn get_rng(&self, node: NodeHandle) -> Option<RNG> {
        self.nodes.borrow()[node.0].unique_part.rng.clone()
    }

    pub fn is_visited(&self, node: NodeHandle) -> bool {
        self.nodes.borrow()[node.0].visited
    }

    pub fn get_custom_data<T: ValidCustomData + 'static>(&self, node: NodeHandle) -> Option<T> {
        let binding = self.nodes.borrow();
        let reference = binding[node.0].unique_part.custom_data.as_ref() as &dyn Any;

        let reference_to_t = reference.downcast_ref();
        reference_to_t.cloned()
    }
}
