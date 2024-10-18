use qts_rust::aeqts::aeqts;
use qts_rust::case::{case_1, case_2, case_3};
use qts_rust::config::Config;
use qts_rust::qts::qts;
use qts_rust::types::Problem;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let config = Config::get().unwrap();

    let problem = Problem::default();
    let items = Arc::new(Mutex::new(Vec::new()));
    let capacity = {
        let mut items = items.lock().unwrap();
        if config.problem.case == 1 {
            case_1(&mut items, problem) as f64
        } else if config.problem.case == 2 {
            case_2(&mut items, problem) as f64
        } else if config.problem.case == 3 {
            case_3(&mut items, problem) as f64
        } else {
            panic!("Invalid case");
        }
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
                i,
            );
            aeqts(
                &items,
                capacity,
                config.algorithm.max_gen,
                config.algorithm.n_neighbors,
                i,
            );
            println!("thread[{}] done", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
