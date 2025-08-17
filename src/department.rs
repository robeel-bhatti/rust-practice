use std::collections::HashMap;

pub fn get_employees(department: String) -> String {
    let empl_map: HashMap<String, String> = init_data();
    let mut res: Vec<String> = vec![];

    for (key, value) in empl_map.iter() {
        if *value == department {
            res.push(key.to_string());
        }
    }

    res.join(", ").to_string()
}



fn init_data() -> HashMap<String, String> {
    let mut empl: HashMap<String, String> = HashMap::new();
    empl.insert("Bob".to_string(), "Engineering".to_string());
    empl.insert("Jane".to_string(), "Sales".to_string());
    empl.insert("Sally".to_string(), "Sales".to_string());
    empl.insert("Mike".to_string(), "Engineering".to_string());
    empl
}
