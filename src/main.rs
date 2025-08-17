use std::collections::HashMap;

fn main() {
    let mut even: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let mut odd: Vec<i32> = vec![1,2,3,4,5, 6, 7, 8, 9];

    println!("Median of list 1 is {}", get_median(&mut even));
    println!("Mode of list 1 is {}", get_mode(&even));

    println!("Median of list 2 is {}", get_median(&mut odd));
    println!("Mode of list 2  is {}", get_mode(&odd));
}

fn get_median(values: &mut Vec<i32>) -> f64 {
    values.sort();
    let vl = values.len();
    let mid = vl / 2;

    if vl % 2 == 0 {
        (values[mid-1] + values[mid]) as f64 / 2.0
    } else {
        values[mid] as f64
    }
}

fn get_mode(values: &Vec<i32>) -> i32 {
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
