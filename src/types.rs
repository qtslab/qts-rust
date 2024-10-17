use crate::qubit::Qubit;

#[derive(Clone, Default)]
pub struct Item {
    pub value: f64,
    pub weight: f64,
}

pub struct Problem {
    pub size: i32,
    pub capacity: f64,
    pub max_weight: f64,
    pub max_value: f64,
    pub min_weight: f64,
    pub min_value: f64,
}

impl Default for Problem {
    fn default() -> Self {
        Problem {
            size: 20,
            capacity: 0.0,
            max_weight: 10.0,
            max_value: 10.0,
            min_weight: 1.0,
            min_value: 1.0,
        }
    }
}

impl Problem {
    pub fn set_size(mut self, size: i32) {
        self.size = size;
    }
}

// define a alias for a vector of qubits(for a qubit register)
pub type Qubits = Vec<Qubit>;
// define a alias for a vector of bools(for a solution)
pub type Solution = Vec<bool>;
// define a alias for a vector of items(for a problem)
pub type Items = Vec<Item>;
