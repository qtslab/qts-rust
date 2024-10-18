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

    let mut qts_handles = vec![];

    let qts_start_time = std::time::Instant::now();
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
            println!("QTS thread[{}] done", i);
        });
        qts_handles.push(handle);
    }

    for handle in qts_handles {
        handle.join().unwrap();
    }

    let qts_end_time = std::time::Instant::now();
    let qts_duration = qts_end_time.duration_since(qts_start_time);

    // AE-QTS
    let mut aeqts_handles = vec![];
    let aeqts_start_time = std::time::Instant::now();
    for i in 0..config.test.count {
        let items = Arc::clone(&items);
        let handle = thread::spawn(move || {
            let items = items.lock().unwrap();
            aeqts(
                &items,
                capacity,
                config.algorithm.max_gen,
                config.algorithm.n_neighbors,
                i,
            );
            println!("AEQTS thread[{}] done", i);
        });
        aeqts_handles.push(handle);
    }

    for handle in aeqts_handles {
        handle.join().unwrap();
    }

    let aeqts_end_time = std::time::Instant::now();
    let aeqts_duration = aeqts_end_time.duration_since(aeqts_start_time);

    println!("QTS duration: {:?}", qts_duration);
    println!("AEQTS duration: {:?}", aeqts_duration);
}
