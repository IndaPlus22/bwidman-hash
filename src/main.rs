mod hash_map;

use std::io;
use hash_map::HashMap;

type Entry = Vec<String>;

struct Table {
    headers: Entry, // Titles for every column header
    entry_indices: Vec<HashMap<Vec<usize>>>, // Hash maps for every column, containing indices for the respective entries having that value in that column
    // Problem: during insertion, must use search() to get reference to Vec of indices then push(index), if HashMap::contains(col_value)
    entries: Vec<Entry>, // Entries with values for every column
}

impl Table {
    fn new(file_path: &str) -> Self {
        // Open file


        // Read data
        // First line contains the headers

        // Every row is an entry with comma separated column values


        Self { headers, entry_indices: , entries:  }
    }
    
    fn select(&self, parameters: Vec<&str>) {
        // Only "select", just print all entries
        if parameters.len() == 1 {
            for entry in self.entries {
                print_entry(entry);
            }
        }

        println!("Matched result: {}", data.search(parameters[1]).unwrap());
    }
}

fn print_entry(entry: Entry) {
    for value in entry {
        print!("{value}\t");
    }
    println!();
}

fn main() {
    // Get stored database file
    let mut data = Table::new("data.csv");

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
            "select" => data.select(parameters),
            "create" => todo!(),
            _ => (),
        }
    }
}
