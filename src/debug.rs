use std::f64::consts::SQRT_2;

use crate::types::{Items, Qubits, Solution};

pub fn print_items(items: &Items) {
    for i in 0..items.len() {
        println!(
            "Item {}: value = {:.3}, weight = {:.3}",
            i, items[i as usize].value, items[i as usize].weight
        );
    }
}

pub fn print_best_worst(best: &Solution, worst: &Solution) {
    println!("Best solution: ");
    for i in 0..best.len() {
        print!("{} ", best[i as usize]);
    }
    println!();
    println!("Worst solution: ");
    for i in 0..worst.len() {
        print!("{} ", worst[i as usize]);
    }
    println!();
}

pub fn print_qubits(qubits: &Qubits) {
    for i in 0..qubits.len() {
        println!(
            "Qubit {}: alpha = {:.3}, beta = {:.3}",
            i,
            qubits[i as usize].alpha / SQRT_2,
            qubits[i as usize].beta / SQRT_2
        );
    }
}

pub fn print_solution(items: &Items, solution: &Solution) {
    let mut total_value: f64 = 0.0;
    let mut total_weight: f64 = 0.0;
    for i in 0..solution.len() {
        if solution[i as usize] {
            total_value += items[i as usize].value;
            total_weight += items[i as usize].weight;
        }
    }
    println!(
        "Total value: {:.3}, Total weight: {:.3}",
        total_value, total_weight
    );
}
