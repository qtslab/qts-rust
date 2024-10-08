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
    for qubit in qubits {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen();
        if r < qubit.alpha.powi(2) {
            solution.push(true);
        } else {
            solution.push(false);
        }
    }

    solution
}

pub fn adjust_solution(items: &Items, solution: &Solution, capacity: f64) -> Solution {
    let mut weight = calculate_weights(items, solution);
    let mut rng = rand::thread_rng();
    let mut adjusted_solution = solution.clone();
    while weight > capacity {
        let rand_index = rng.gen_range(0..items.len());
        if adjusted_solution[rand_index] {
            adjusted_solution[rand_index] = false;
            weight -= items[rand_index].weight;
        }
    }

    adjusted_solution
}

pub fn update_qubits(best_sol: Solution, worst_sol: Solution, qubits: &mut Qubits) -> i32 {
    let theta: f64 = 0.01 * std::f64::consts::PI;
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

pub fn sort_solution(items: &Items, solutions: Vec<Solution>, n_neighbors: i32) -> Vec<Solution> {
    let mut sorted_solutions: Vec<Solution> = solutions.clone();
    let mut values: Vec<f64> = Vec::new();
    for solution in &solutions {
        values.push(calculate_values(items, solution));
    }

    for i in 0..n_neighbors {
        for j in i..n_neighbors {
            if values[i as usize] < values[j as usize] {
                let temp = values[i as usize];
                values[i as usize] = values[j as usize];
                values[j as usize] = temp;

                let temp = sorted_solutions[i as usize].clone();
                sorted_solutions[i as usize] = sorted_solutions[j as usize].clone();
                sorted_solutions[j as usize] = temp;
            }
        }
    }

    sorted_solutions
}
