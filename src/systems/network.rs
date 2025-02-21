use crate::Node;

#[allow(dead_code)]
pub struct Layer {
    pub nodes: Vec<Node>,
}

#[allow(dead_code)]
pub struct Network {
    pub layers: Vec<Layer>,
    pub inputs: Vec<f64>,
    pub outputs: Vec<f64>,
}

#[allow(dead_code)]
impl Network {
    pub fn new() -> Network {
        Network {layers: Vec::new(), inputs: Vec::new(), outputs: Vec::new()}
    }

    pub fn update_input(&mut self, index: usize, new_value: f64) -> f64 {
        let old_value: f64 = *self.inputs.get(index).expect("[ERROR] Failed to get old value");
        self.inputs.remove(index);
        self.inputs.insert(index, new_value);
        old_value
    }

    pub fn calc(&mut self) {
        for node in *self.layers.get_mut(0).expect("[ERROR] Failed to get first layer").nodes.into_iter() {
            node.calc(self.inputs.into_iter().sum());
        }
    }
}
