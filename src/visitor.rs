use crate::{
    layer::{Layer, LayerResult},
    node_heap::{NodeHandle, NodeHeap},
};

fn visit(
    node: NodeHandle,
    best_final_node_cost: &mut u32,
    current_cost: u32,
    path: Vec<String>,
    node_heap: &'static NodeHeap,
    layers: &[Box<Layer>],
    doctor_strange_counter: &mut u32,
) {
    node_heap.mark_visited(node);
    if layers.is_empty() {
        if current_cost < *best_final_node_cost || current_cost == 0 || current_cost == 1 {
            println!("New final node with cost {current_cost} and path {path:?}");

            if current_cost < *best_final_node_cost {
                *best_final_node_cost = current_cost;
            }
            eprintln!(
                "New best final node {current_cost}  ({doctor_strange_counter} branches tested)"
            );

            *doctor_strange_counter += 1;
        }
        return;
    }

    for LayerResult {
        child,
        layer_cost,
        layer_path,
    } in layers[0](node_heap, node, *best_final_node_cost - current_cost)
    {
        if node_heap.is_visited(child) {
            if node_heap.best_cost(child) <= current_cost + layer_cost {
                continue;
            }
        }
        node_heap.mark_visited(child);

        // this is why its crucially important that layers are guaranteed sorted
        if current_cost + layer_cost > *best_final_node_cost && current_cost + layer_cost > 1 {
            *doctor_strange_counter += 1;
            break;
        }

        let mut path = path.clone();

        if let Some(lp) = layer_path {
            path.push(lp);
        }

        node_heap.update_best_cost(node, current_cost + layer_cost);
        visit(
            child,
            best_final_node_cost,
            current_cost + layer_cost,
            path,
            node_heap,
            &layers[1..],
            doctor_strange_counter,
        );
    }
}

pub fn run_visitor(layers: &[Box<Layer>]) {
    let nh = Box::leak(Box::new(NodeHeap::default()));

    let root_node = nh.get_or_construct_node("root", None);

    let mut best_final_node_cost = u32::MAX;

    let mut doctor_strange_counter = 0;

    visit(
        root_node,
        &mut best_final_node_cost,
        0,
        vec![],
        nh,
        layers,
        &mut doctor_strange_counter,
    )
}
