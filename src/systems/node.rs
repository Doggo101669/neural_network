use rand::prelude::*;

#[allow(dead_code)]
pub struct Node {
    pub weight: f64,
    pub bias: f64,
    pub output: f64,
}

#[allow(dead_code)]
impl Node {
    pub fn new(weight: f64, bias: f64) -> Node {
        Node {weight, bias, output: 0.0}
    }

    pub fn new_rand() -> Node {
        Node::new(rand::random::<f64>() % 1.0, rand::random::<f64>() % 1.0)
    }

    pub fn calc(&mut self, input: f64) -> f64 {
        self.output = input * self.weight;
        self.output += self.bias;
        self.output
    }
}
