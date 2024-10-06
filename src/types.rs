use std::f64::consts::SQRT_2;

#[derive(Clone)]
pub struct Qubit {
    pub alpha: f64,
    pub beta: f64,
}

impl Default for Qubit {
    fn default() -> Self {
        Qubit {
            alpha: 1.0 / SQRT_2,
            beta: 1.0 / SQRT_2,
        }
    }
}

#[derive(Clone, Default)]
pub struct Item {
    pub value: f64,
    pub weight: f64,
}

impl Item {
    fn new(value: f64, weight: f64) -> Item {
        Item { value, weight }
    }
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

// define a alias for a vector of qubits(for a qubit register)
pub type Qubits = Vec<Qubit>;
// define a alias for a vector of bools(for a solution)
pub type Solution = Vec<bool>;
// define a alias for a vector of items(for a problem)
pub type Items = Vec<Item>;
