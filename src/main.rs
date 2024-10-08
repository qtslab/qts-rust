use qts_rust::case::case_1;
use qts_rust::qts::qts;
use qts_rust::types::{Items, Problem};

use qts_rust::debug::print_items;

fn main() {
    let mut items: Items = Vec::new();
    let problem = Problem::default();
    let capacity: f64 = case_1(&mut items, problem) as f64;

    print_items(&items);

    let max_gen: i32 = 100;
    let n_neighbors: i32 = 100;
    qts(&items, capacity, max_gen, n_neighbors);
}
