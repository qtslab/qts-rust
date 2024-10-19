use crate::quantum::calculate_values;
use crate::record::Record;
use crate::types::{Items, Solution};

pub fn dp(items: &Items, capacity: f64, test_count: i32) -> Solution {
    let mut best_fit: Solution = vec![false; items.len()];
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; (capacity + 1.0) as usize]; items.len()];
    let mut record = Record::new(format!("csv/dp/{}.csv", test_count));

    for i in 0..items.len() {
        for j in 0..(capacity + 1.0) as usize {
            if i == 0 {
                dp[i][j] = if items[i].weight <= j as f64 {
                    items[i].value
                } else {
                    0.0
                };
            } else {
                dp[i][j] = if items[i].weight <= j as f64 {
                    dp[i - 1][j]
                        .max(dp[i - 1][(j - items[i].weight as usize) as usize] + items[i].value)
                } else {
                    dp[i - 1][j]
                };
            }
        }
    }

    let mut j = capacity as usize;
    for i in (0..items.len()).rev() {
        if i == 0 {
            best_fit[i] = dp[i][j] > 0.0;
        } else if dp[i][j] != dp[i - 1][j] {
            best_fit[i] = true;
            j -= items[i].weight as usize;
        }
    }

    record.add_global_best(calculate_values(items, &best_fit), best_fit.clone());
    let _ = record.write_file_global_best();
    best_fit
}
