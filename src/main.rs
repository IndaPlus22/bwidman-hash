use std::io;

struct HashMap<T> {
    keys: Vec<T>,
    values: Vec<T>,
}

impl<T> HashMap<T> {
    fn new() -> Self {
        const START_CAPACITY: usize = 50;
        Self {
            keys: Vec::with_capacity(START_CAPACITY),
            values: Vec::with_capacity(START_CAPACITY),
        }
    }

    fn insert(&mut self, key: T, value: T) {
        todo!()
    }

    fn delete(&mut self, key: T) {
        todo!()
    }

    fn search(&mut self, key: T) -> T {
        todo!()
    }
}

fn main() {
    // Get stored database file
    let mut data: HashMap<u32> = HashMap::new();

    println!("Welcome bwidman's secret database!");

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command == "quit" {
            break;
        }

        let parameters: Vec<&str> = command.split(' ').collect();
        match parameters[0] {
            "insert" => todo!(),
            "delete" => todo!(),
            "select" => todo!(),
            _ => (),
        }
    }
}
