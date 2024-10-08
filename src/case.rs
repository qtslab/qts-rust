use crate::types::{Items, Problem};

use rand::Rng;

pub fn case_I(items: &mut Items, mut problem: Problem) -> f64 {
    // weight = random(min_weight, max_weight)
    // value = weight + 5
    let mut rng = rand::thread_rng();
    items.resize(problem.size as usize, Default::default());
    for i in 0..problem.size {
        items[i as usize].weight = rng.gen_range(problem.min_weight..problem.max_weight);
        items[i as usize].value = items[i as usize].weight + 5.0;
        problem.capacity += items[i as usize].weight;
    }

    return problem.capacity / 2.0;
}

pub fn case_II(items: &mut Items, mut problem: Problem) -> f64 {
    // weight = i%10 + 1
    // value = random(min_value, max_value)
    let mut rng = rand::thread_rng();
    items.resize(problem.size as usize, Default::default());
    for i in 0..problem.size {
        items[i as usize].weight = i as f64 / 10.0 + 1.0;
        items[i as usize].value = rng.gen_range(problem.min_value..problem.max_value);
        problem.capacity += items[i as usize].weight;
    }

    return problem.capacity / 2.0;
}

pub fn case_III(items: &mut Items, mut problem: Problem) -> f64 {
    // weight = i%10 + 1
    // value = weight + 5
    items.resize(problem.size as usize, Default::default());
    for i in 0..problem.size {
        items[i as usize].weight = i as f64 % 10.0 + 1.0;
        items[i as usize].value = items[i as usize].weight + 5.0;
        problem.capacity += items[i as usize].weight;
    }

    return problem.capacity / 2.0;
}
