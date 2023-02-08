mod hash_map;

use std::io;
use hash_map::HashMap;

fn main() {
    // Get stored database file
    let mut data: HashMap<u32> = HashMap::new();

    println!("Welcome to bwidman's secret database!");

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        
        if command.trim() == "quit" {
            break;
        }

        let parameters: Vec<&str> = command.trim().split(' ').collect();
        
        match parameters[0] {
            "insert" => data.insert(parameters[1], parameters[2].parse::<u32>().unwrap()),
            "delete" => data.delete(parameters[1]),
            "select" => println!("Matched result: {}", data.search(parameters[1]).unwrap()),
            _ => (),
        }
    }
}
