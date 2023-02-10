// Author: Benjamin Widman
mod hash_map;

use std::{io::{self, Write, Read}, fs::File, collections::VecDeque};
use hash_map::HashMap;

type Entry = Vec<String>;

struct Table {
    headers: Entry, // Titles for every column header
    entry_indices: Vec<HashMap<Vec<usize>>>, // Hash maps for every column, containing indices for the respective entries having the key as the value in that column
    // Problem: during insertion, must use search() to get reference to Vec of indices then push(index), if HashMap::contains(col_value)
    entries: Vec<Entry>, // Entries with values for every column
    file: File,
    file_lines: Vec<String>,
    deleted_lines: Vec<usize>,
}

impl Table {
    fn new(file_path: &str) -> Self {
        // Open writeable file
        match File::options().read(true).write(true).open(file_path) {
            Ok(file) => return Table::read_file(file),
            Err(_error) => return Table::create_file(file_path),
        }
    }

    fn create_file(file_path: &str) -> Self {
        println!("No previous table stored in {file_path}\n");
        let mut file = File::options().read(true).write(true).create(true).open(file_path).unwrap();

        println!("Creating new table");
        println!("Write the title of every column header seperated with commas (,) without spaces:");
        let mut headers_string = String::new();
        io::stdin().read_line(&mut headers_string).unwrap();

        file.write_all(headers_string.as_bytes()).unwrap();

        let headers: Entry = headers_string.trim().split(',')
            .map(|s| s.to_string())
            .collect();

        let mut entry_indices = Vec::with_capacity(headers.len());
        for _i in 0..headers.len() {
            entry_indices.push(HashMap::new());
        }

        let file_lines = vec![headers_string.trim().to_string()];

        Self { headers, entry_indices, entries: vec![], file, file_lines, deleted_lines: vec![] }
    }

    fn read_file(mut file: File) -> Self {
        // Read data
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut lines: VecDeque<&str> = content.lines().collect();

        // First line contains the headers
        let headers: Entry = lines.pop_front().unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let mut entry_indices: Vec<HashMap<Vec<usize>>> = Vec::with_capacity(headers.len());
        // Fill with hash maps
        for _i in 0..headers.len() {
            entry_indices.push(HashMap::new());
        }

        let mut entries = vec![Vec::with_capacity(headers.len()); lines.len()];

        // Every row is an entry with comma separated column values
        for line in lines {
            let entry: Entry = line.split(',')
                .map(|s| s.to_string())
                .collect();
                
            let entry_index = entries.len();

            Table::insert_entry_index(&entry, entry_index, &mut entry_indices);
            
            entries.push(entry);
        }

        let file_lines = content.lines().map(|s| s.to_string()).collect();

        Self { headers, entry_indices, entries, file, file_lines, deleted_lines: vec![] }
    }

    // Add index look-up for the value of every column
    fn insert_entry_index(entry: &Entry, entry_index: usize, entry_indices: &mut Vec<HashMap<Vec<usize>>>) {
        for i in 0..entry_indices.len() {
            // Check if slot with indices for that column value already exists
            match entry_indices[i].search(entry[i].as_str()) {
                Some(hash_slot) => hash_slot.push(entry_index), // Already exists, push new index to slot
                None => entry_indices[i].insert(entry[i].as_str(), vec![entry_index]), // Doesn't exist, create one with index
            }
        }
    }
    
    // Inserts entry with specified column values
    fn insert(&mut self, parameters: Vec<String>) {
        if parameters.len() != self.headers.len() {
            println!("Too many or too few columns entered!");
            return;
        }
        // Write to file
        // writeln!(self.file, "{}", parameters.join(",")).unwrap();
        self.file_lines.push(parameters.join(","));

        // Input into program data
        let entry_index = self.entries.len();
        Table::insert_entry_index(&parameters, entry_index, &mut self.entry_indices);

        self.entries.push(parameters);
    }
    
    // Deletes entries matching query
    fn delete(&mut self, parameters: Vec<String>) {
        // Find matches for specified entry
        let matches = self.matches(parameters);

        // Delete matches in file by getting it's content and only setting the other entries back again
        self.deleted_lines.append(&mut matches.clone());

        // Delete matches in program data
        for &entry_index in &matches {
            let entry = &self.entries[entry_index];
            
            // Remove entry index from all column hash map slots
            for i in 0..entry.len() {
                let indices = self.entry_indices[i].search(entry[i].as_str()).unwrap();
                let del_index = indices.iter().position(|&x| x == entry_index).unwrap();
                indices.remove(del_index);
            }
            self.entries[entry_index] = vec![]; // Can't remove as it would disrupt the indices
        }
    }
    
    fn select(&mut self, parameters: Vec<String>) {
        // Only "select", just print all entries
        if parameters.len() == 0 {
            print_entry(&self.headers);
            println!("---------------------------------");
            for entry in &self.entries {
                print_entry(entry);
            }
            return;
        }

        // Find matches for specified entry
        let matches = self.matches(parameters);

        print_entry(&self.headers);
        println!("---------------------------------");
        // Print all matches
        for entry_index in matches {
            print_entry(&self.entries[entry_index]);
        }
    }

    // Returns indices of 
    fn matches(&mut self, filter: Vec<String>) -> Vec<usize> {
        let filter1: Vec<String> = filter[0].split('=')
            .map(|s| s.to_string())
            .collect();
        
        match self.headers.iter().position(|x| *x == filter1[0]) {
            Some(header_index) => {
                match self.entry_indices[header_index].search(filter1[1].as_str()) {
                    Some(indices) => return indices.clone(),
                    None => return vec![],
                }
            },
            None => return vec![],
        }
    }
}

fn print_entry(entry: &Entry) {
    print!("| ");
    for value in entry {
        if value.len() <= 5 {
            print!("{value}\t\t| ");
        } else {
            print!("{value}\t| ");
        }
    }
    println!();
}

fn main() {
    // Get stored database file
    let mut data = Table::new("data.csv");

    println!("Welcome to bwidman's secret database!\n");
    println!("Current table layout:");
    print_entry(&data.headers);

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        
        if query.trim() == "quit" {
            break;
        }

        let mut parameters: Vec<String> = query.trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        
        let command = parameters.remove(0);
        
        match command.as_str() {
            "insert" => data.insert(parameters),
            "delete" => data.delete(parameters),
            "select" => data.select(parameters),
            _ => (),
        }
    }
    data.file.set_len(0).unwrap(); // Clear file

    writeln!(data.file, "{}", data.file_lines[0]).unwrap(); // Insert header

    for i in 1..data.file_lines.len() {
        if !data.deleted_lines.contains(&(i - 1)) {
            writeln!(data.file, "{}", data.file_lines[i]).unwrap();
        }
    }
}
