use qts_rust::case::case_1;
use qts_rust::qts::qts;
use qts_rust::types::{Items, Problem};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let test_times: i32 = 1000;
    let max_gen: i32 = 1000;
    let n_neighbors: i32 = 10;

    let problem = Problem::default();
    let items = Arc::new(Mutex::new(Vec::new()));
    let capacity = {
        let mut items = items.lock().unwrap();
        case_1(&mut items, problem) as f64
    };

    let mut handles = vec![];

    for i in 0..test_times {
        let items = Arc::clone(&items);
        let handle = thread::spawn(move || {
            let items = items.lock().unwrap();
            qts(&items, capacity, max_gen, n_neighbors);
            println!("thread[{}] done", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
