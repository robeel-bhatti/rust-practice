use std::collections::HashMap;

pub fn get_median(values: &mut Vec<i32>) -> f64 {
    values.sort();
    let vl = values.len();
    let mid = vl / 2;

    if vl % 2 == 0 {
        (values[mid-1] + values[mid]) as f64 / 2.0
    } else {
        values[mid] as f64
    }
}

pub fn get_mode(values: &Vec<i32>) -> i32 {
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    for &i in values {
        *freq_map.entry(i).or_insert(0) += 1;
    }

    let max_freq: &i32 = freq_map.values().max().unwrap_or(&0);

    for (k, v) in &freq_map {
        if *v == *max_freq {
            return *k;
        }
    }

    *max_freq
}