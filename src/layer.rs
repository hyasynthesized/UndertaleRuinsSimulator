use crate::node_heap::{NodeHandle, NodeHeap};

pub struct LayerResult {
    pub child: NodeHandle,
    pub layer_cost: u32,
    pub layer_path: Option<String>,
}

pub type LayerIter = Box<dyn Iterator<Item = LayerResult>>;
pub type Layer = dyn Fn(&'static NodeHeap, NodeHandle, u32) -> LayerIter;
