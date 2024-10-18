use crate::quantum::{
    adjust_solution, calculate_values, calculate_weights, measure, update_qubits,
};
use crate::types::{Items, Qubits, Solution};

use crate::record::Record;

pub fn qts(
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
    let mut best_solution: Solution = vec![false; items.len()];
    let mut worst_solution: Solution = vec![false; items.len()];
    let mut record = Record::new(format!("csv/qts/{}.csv", test_count));
    for i in 0..max_gen {
        // println!("Generation {}/{}", i+1, max_gen);
        for j in 0..n_neighbors {
            neighbors[j as usize] = measure(&qubits);
            adjust_solution(items, &mut neighbors[j as usize], capacity);
            if j == 0 {
                best_solution = neighbors[j as usize].clone();
                worst_solution = neighbors[j as usize].clone();
            }

            if calculate_values(items, &neighbors[j as usize])
                > calculate_values(items, &best_solution)
            {
                best_solution = neighbors[j as usize].clone();
            } else if calculate_values(items, &neighbors[j as usize])
                < calculate_values(items, &worst_solution)
            {
                worst_solution = neighbors[j as usize].clone();
            }
        }

        if calculate_values(items, &best_solution) > calculate_values(items, &best_fit) {
            best_fit = best_solution.clone();
        }

        record.add_iteration(
            i,
            calculate_values(items, &best_fit),
            calculate_weights(items, &best_fit),
            best_fit.clone(),
            qubits.clone(),
        );

        update_qubits(best_solution.clone(), worst_solution.clone(), &mut qubits);
    }

    let _ = record.write_file();
    best_fit
}
