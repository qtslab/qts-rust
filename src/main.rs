use qts_rust::aeqts::aeqts;
use qts_rust::case::case_1;
use qts_rust::config::Config;
use qts_rust::qts::qts;
use qts_rust::types::{Items, Problem};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let config = Config::get().unwrap();

    let problem = Problem::default();
    let items = Arc::new(Mutex::new(Vec::new()));
    let capacity = {
        let mut items = items.lock().unwrap();
        case_1(&mut items, problem) as f64
    };

    let mut handles = vec![];

    for i in 0..config.test.count {
        let items = Arc::clone(&items);
        let handle = thread::spawn(move || {
            let items = items.lock().unwrap();
            qts(
                &items,
                capacity,
                config.algorithm.max_gen,
                config.algorithm.n_neighbors,
            );
            aeqts(
                &items,
                capacity,
                config.algorithm.max_gen,
                config.algorithm.n_neighbors,
            );
            println!("thread[{}] done", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
