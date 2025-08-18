mod median_and_mode;
mod pig_latin;
mod department;

use median_and_mode::get_median;
use median_and_mode::get_mode;
use pig_latin::to_pig_latin;
use department::get_employees;

fn main() {
    let mut even: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    let mut odd: Vec<i32> = vec![1,2,3,4,5, 6, 7, 8, 9];

    println!("Median of list 1 is {}", get_median(&mut even));
    println!("Mode of list 1 is {}", get_mode(&even));

    println!("Median of list 2 is {}", get_median(&mut odd));
    println!("Mode of list 2  is {}", get_mode(&odd));

    let first: String = String::from("first");
    let apple: String = String::from("apple");

    assert_eq!(to_pig_latin(first), "irstfay");
    assert_eq!(to_pig_latin(apple), "applehay");

    let eng: String = String::from("Engineering");
    let sales: String = String::from("Sales");

    println!("Engineering: {}", get_employees(eng));
    println!("Sales: {}", get_employees(sales));
}


