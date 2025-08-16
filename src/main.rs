use std::collections::HashMap;

fn main() {
    let mut even: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let mut odd: Vec<i32> = vec![1,2,3,4,5, 6, 7, 8, 9];

    assert_eq!(get_median(&mut even), 5.5);
    assert_eq!(get_median(&mut odd), 5.0);

    assert_eq!(get_mode(&even), -1);
    assert_eq!(get_mode(&odd), -1);
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
    let mut count: HashMap<i32, i32> = HashMap::new();

    for &i in values {
        *count.entry(i).or_insert(0) += 1;
    }

    let mut highest_key = -1;
    let mut highest_value = -1;

    for (k, v) in &count {
        if *v > highest_value {
            highest_value = *v;
            highest_key = *k;
        }
    }

    highest_key
}
