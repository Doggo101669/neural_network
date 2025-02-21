use systems::{network::*, node::*};

mod systems;

fn main() {
    let mut network = Network::new();
    network.inputs.push(0.0);
    network.inputs.push(0.0);
    network.layers.push(
        Layer {
            nodes: vec![
                Node::new_rand(),
                Node::new_rand(),
                Node::new_rand(),
            ],
        }
    );
    network.layers.push(
        Layer {
            nodes: vec![
                Node::new_rand(),
                Node::new_rand(),
                Node::new_rand(),
            ]
        }
    );
    network.outputs.push(0.0);
    network.outputs.push(0.0);
}
