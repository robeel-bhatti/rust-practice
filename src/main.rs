mod median_and_mode;

use median_and_mode::get_median;
use median_and_mode::get_mode;

fn main() {
    let mut even: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let mut odd: Vec<i32> = vec![1,2,3,4,5, 6, 7, 8, 9];

    println!("Median of list 1 is {}", get_median(&mut even));
    println!("Mode of list 1 is {}", get_mode(&even));

    println!("Median of list 2 is {}", get_median(&mut odd));
    println!("Mode of list 2  is {}", get_mode(&odd));
}


