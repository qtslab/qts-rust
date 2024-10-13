use crate::types::{Items, Qubits, Solution};

use rand::Rng;

pub fn calculate_weights(items: &Items, solution: &Solution) -> f64 {
    let mut weight: f64 = 0.0;
    for i in 0..items.len() {
        if solution[i] {
            weight += items[i].weight;
        }
    }
    weight
}

pub fn calculate_values(items: &Items, solution: &Solution) -> f64 {
    let mut value: f64 = 0.0;
    for i in 0..items.len() {
        if solution[i] {
            value += items[i].value;
        }
    }
    value
}

pub fn measure(qubits: &Qubits) -> Solution {
    let mut solution: Solution = Vec::new();
    let mut rng = rand::thread_rng();
    for qubit in qubits {
        let r: f64 = rng.gen();
        if r < qubit.alpha.powi(2) {
            solution.push(true);
        } else {
            solution.push(false);
        }
    }

    solution
}

pub fn adjust_solution(items: &Items, solution: &mut Solution, capacity: f64) {
    let mut weight = calculate_weights(items, solution);
    let mut rng = rand::thread_rng();
    while weight > capacity {
        let rand_index = rng.gen_range(0..items.len());
        if solution[rand_index] {
            solution[rand_index] = false;
            weight -= items[rand_index].weight;
        }
    }
}

pub fn update_qubits(best_sol: Solution, worst_sol: Solution, qubits: &mut Qubits) -> i32 {
    let theta: f64 = 0.01 * std::f64::consts::PI;
    for i in 0..qubits.len() {
        let mut mod_signal: i32 = best_sol[i] as i32 - worst_sol[i] as i32;
        if qubits[i].alpha * qubits[i].beta < 0.0 {
            mod_signal *= -1;
        }

        let new_alpha = qubits[i].alpha * (theta * mod_signal as f64).cos()
            - qubits[i].beta * (theta * mod_signal as f64).sin();
        let new_beta = qubits[i].beta * (theta * mod_signal as f64).sin()
            + qubits[i].alpha * (theta * mod_signal as f64).cos();

        qubits[i].alpha = new_alpha;
        qubits[i].beta = new_beta;
    }

    0
}

pub fn update_qubits_with_angle(
    best_sol: Solution,
    worst_sol: Solution,
    qubits: &mut Qubits,
    angle: f64,
) -> i32 {
    let theta: f64 = angle * std::f64::consts::PI;
    for i in 0..qubits.len() {
        let mut mod_signal: i32 = best_sol[i] as i32 - worst_sol[i] as i32;
        if qubits[i].alpha * qubits[i].beta < 0.0 {
            mod_signal *= -1;
        }

        qubits[i].alpha = qubits[i].alpha * (theta * mod_signal as f64).cos()
            - qubits[i].beta * (theta * mod_signal as f64).sin();
        qubits[i].beta = qubits[i].beta * (theta * mod_signal as f64).sin()
            + qubits[i].alpha * (theta * mod_signal as f64).cos();
    }

    0
}

pub fn sort_solution(items: &Items, solutions: Vec<Solution>) -> Vec<Solution> {
    let mut sorted_solutions = solutions.clone();
    sorted_solutions.sort_by(|a, b| {
        calculate_values(items, b)
            .partial_cmp(&calculate_values(items, a))
            .unwrap()
    });
    sorted_solutions
}
