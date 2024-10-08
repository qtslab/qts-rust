use qts_rust::case::case_I;
use qts_rust::qts::qts;
use qts_rust::types::{Items, Problem};

use qts_rust::debug::print_items;

fn main() {
    let mut items: Items = Vec::new();
    let problem = Problem::default();
    let capacity: f64 = case_I(&mut items, problem) as f64;

    print_items(&items);

    let max_gen: i32 = 10;
    let n_neighbors: i32 = 10;
    qts(&items, capacity, max_gen, n_neighbors);
}
