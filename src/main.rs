use std::io;

#[derive(Clone)]
struct Element<T>
where T: Copy {
    key: String,
    value: T
}

struct HashMap<T>
where T: Copy {
    elements: Vec<Option<Element<T>>>
}

impl<T> HashMap<T>
where T: Copy {
    fn new() -> Self {
        const START_CAPACITY: usize = 50;
        
        Self { elements: vec![None; START_CAPACITY] }
    }

    fn insert(&mut self, key: &str, value: T) {
        const MAX_CAPACITY: f32 = 0.75;

        let index = self.hash_code(key);
        self.elements[index] = Some(Element{ key: key.to_string(), value });
    }

    fn delete(&mut self, key: &str) {
        let index = self.hash_code(key);
        self.elements[index] = None;
    }

    fn search(&mut self, key: &str) -> Option<T> {
        let index = self.hash_code(key);

        return match self.elements[index].clone() {
            Some(e) => Some(e.value),
            None => None
        }
    }

    fn hash_code(&self, key: &str) -> usize {
        let key_value: usize = key.as_bytes()
            .iter()
            .map(|x| *x as usize)
            .sum();
        return key_value * 17 % self.elements.len();
    }
}

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
