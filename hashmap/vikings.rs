use std::collections::HashMap;


#[derive(Debug, Eq, Hash, PartialEq)]


struct Viking {
    name: String,
    country: String,
}


impl Viking {
    
    // create new viking 
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use hashmap to store viking's health points.
    let vikings: HashMap<Viking, i32> = HashMap::from([
    (Viking::new("Einar", "Norway"), 25),
    (Viking::new("Olaf", "Denmark"), 25),
    (Viking::new("Harald", "Iceland"), 12),
    ]);
    
    
    // Use derived implementation to print the status of the vikings 
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
