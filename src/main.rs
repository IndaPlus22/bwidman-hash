mod hash_map;

use std::{io::{self, Write, Read}, fs::File, collections::VecDeque};
use hash_map::HashMap;

type Entry = Vec<String>;

struct Table {
    headers: Entry, // Titles for every column header
    entry_indices: Vec<HashMap<Vec<usize>>>, // Hash maps for every column, containing indices for the respective entries having that value in that column
    // Problem: during insertion, must use search() to get reference to Vec of indices then push(index), if HashMap::contains(col_value)
    entries: Vec<Entry>, // Entries with values for every column
    file: File,
}

impl Table {
    fn new(file_path: &str) -> Self {
        // Open file
        match File::open(file_path) {
            Ok(file) => return Table::read_file(file),
            Err(_error) => return Table::create_file(file_path),
        }
    }

    fn create_file(file_path: &str) -> Self {
        println!("No previous table stored in {file_path}\n");
        let mut file = File::create(file_path).unwrap();

        println!("Creating new table.");
        println!("Write the title of every column header seperated with commas (,) without spaces:");
        let headers_string = String::new();
        io::stdin().read_line(&mut headers_string);

        file.write_all(headers_string.as_bytes());

        let headers: Entry = headers_string.split(',')
            .map(|s| s.to_string())
            .collect();

        let entry_indices = Vec::with_capacity(headers.len());
        for i in 0..headers.len() {
            entry_indices.push(HashMap::new());
        }

        Self { headers, entry_indices, entries: vec![], file }
    }

    fn read_file(file: File) -> Self {
        // Read data
        let mut content = String::new();
        file.read_to_string(&mut content);
        let lines: VecDeque<&str> = content.lines().collect();

        // First line contains the headers
        let headers: Entry = lines.pop_front().unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let entry_indices = Vec::with_capacity(headers.len());
        for i in 0..headers.len() {
            entry_indices.push(HashMap::new());
        }

        let entries = vec![Vec::with_capacity(headers.len()); lines.len()];

        // Every row is an entry with comma separated column values
        for line in lines {
            let column_values = line.split(',');
            
        }

        Self { headers, entry_indices, entries, file }
    }
    
    fn insert(&self, parameters: Vec<&str>) {

    }
    
    fn delete(&self, parameters: Vec<&str>) {

    }
    
    fn select(&self, parameters: Vec<&str>) {
        // Only "select", just print all entries
        if parameters.len() == 1 {
            for entry in self.entries {
                print_entry(entry);
            }
        }

        // Find matches for specified entry
        
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
