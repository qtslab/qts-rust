use crate::qubit::Qubit;

use serde::Deserialize;

#[derive(Clone, Default)]
pub struct Item {
    pub value: f64,
    pub weight: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Problem {
    pub case: i32,
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
            case: 1,
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

    pub fn set_capacity(&mut self, items: &Items) {
        // capacity = total weights / 2 based on the items provided
        let total_weight: f64 = items.iter().map(|item| item.weight).sum();
        self.capacity = total_weight / 2.0;
    }
}

// define a alias for a vector of qubits(for a qubit register)
pub type Qubits = Vec<Qubit>;
// define a alias for a vector of bools(for a solution)
pub type Solution = Vec<bool>;
// define a alias for a vector of items(for a problem)
pub type Items = Vec<Item>;
