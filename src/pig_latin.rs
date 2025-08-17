


pub fn to_pig_latin(s: String) -> String {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    if vowels.iter().any(|&v| s.starts_with(v)) {
        format!("{}hay", s)
    } else {
        let fc = s.chars().nth(0).unwrap();
        let rest: &str = &s[1..];
        format!("{}{}ay", rest, fc)
    }
}