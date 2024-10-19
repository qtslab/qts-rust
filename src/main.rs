use qts_rust::aeqts::aeqts;
use qts_rust::case::{case_1, case_2, case_3};
use qts_rust::config::Config;
use qts_rust::qts::qts;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut config = Config::get().unwrap();

    let items = Arc::new(Mutex::new(Vec::new()));
    let mut items_lock = items.lock().unwrap();
    if config.problem.case == 1 {
        case_1(&mut items_lock, config.problem.clone());
    } else if config.problem.case == 2 {
        case_2(&mut items_lock, config.problem.clone());
    } else if config.problem.case == 3 {
        case_3(&mut items_lock, config.problem.clone());
    } else {
        panic!("Invalid case");
    }

    config.problem.set_capacity(&mut items_lock);
    drop(items_lock);

    let mut qts_handles = vec![];

    let qts_start_time = std::time::Instant::now();
    for i in 0..config.test.count {
        let items = Arc::clone(&items);
        let config = config.clone();
        let handle = thread::spawn(move || {
            let items = items.lock().unwrap();
            qts(
                &items,
                config.problem.capacity,
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
        let config = config.clone();
        let handle = thread::spawn(move || {
            let items = items.lock().unwrap();
            aeqts(
                &items,
                config.problem.capacity,
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

    let mut dp_handles = vec![];
    let dp_start_time = std::time::Instant::now();
    for i in 0..config.test.count {
        let items = Arc::clone(&items);
        let config = config.clone();
        let handle = thread::spawn(move || {
            let items = items.lock().unwrap();
            qts_rust::dp::dp(&items, config.problem.capacity, i);
            println!("DP thread[{}] done", i);
        });
        dp_handles.push(handle);
    }

    for handle in dp_handles {
        handle.join().unwrap();
    }

    let dp_end_time = std::time::Instant::now();
    let dp_duration = dp_end_time.duration_since(dp_start_time);

    println!("QTS duration: {:?}", qts_duration);
    println!("AEQTS duration: {:?}", aeqts_duration);
    println!("DP duration: {:?}", dp_duration);
}
