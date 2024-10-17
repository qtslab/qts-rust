use crate::quantum::{
    adjust_solution, calculate_values, calculate_weights, measure, update_qubits_with_angle,
};
use crate::types::{Items, Qubits, Solution};

use crate::record::Record;

pub fn aeqts(
    items: &Items,
    capacity: f64,
    max_gen: i32,
    n_neighbors: i32,
    test_count: i32,
) -> Solution {
    let mut qubits: Qubits = vec![Default::default(); items.len()];
    let mut best_fit: Solution = vec![false; items.len()];
    adjust_solution(&items, &mut best_fit, capacity);
    let mut neighbors: Vec<Solution> = vec![vec![false; items.len()]; n_neighbors as usize];
    let mut solutions: Vec<Solution> = vec![vec![false; items.len()]; n_neighbors as usize];
    let mut record = Record::new(format!("csv/ae-qts/{}.csv", test_count));
    for i in 0..max_gen {
        // println!("Generation {}/{}", i+1, max_gen);
        for j in 0..n_neighbors {
            neighbors[j as usize] = measure(&qubits);
            adjust_solution(items, &mut neighbors[j as usize], capacity);
            if j == 0 {
                for k in 0..items.len() {
                    solutions[j as usize][k] = neighbors[j as usize][k];
                }
            }
        }

        // sort solutinos by value
        solutions.sort_by(|a, b| {
            calculate_values(items, a)
                .partial_cmp(&calculate_values(items, b))
                .unwrap()
        });

        if calculate_values(items, &solutions[0]) > calculate_values(items, &best_fit) {
            best_fit = solutions[0].clone();
        }

        record.add_iteration(
            i,
            calculate_values(items, &solutions[0]),
            calculate_weights(items, &solutions[0]),
            best_fit.clone(),
            qubits.clone(),
        );

        for j in 0..solutions.len() / 2 {
            update_qubits_with_angle(
                solutions[j].clone(),
                solutions[solutions.len() - j - 1].clone(),
                &mut qubits,
                0.01 / (j + 1) as f64,
            );
        }
    }

    let _ = record.write_file();
    best_fit
}
