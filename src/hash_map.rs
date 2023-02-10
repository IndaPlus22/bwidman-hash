#[derive(Clone)]
pub struct Element<T>
where T: Clone {
    key: String,
    value: T,
}

pub struct HashMap<T>
where T: Clone {
    slots: Vec<Vec<Element<T>>>,
    fullness: f32,
}

impl<T> HashMap<T>
where T: Clone {
    pub fn new() -> Self {
        const START_CAPACITY: usize = 32;
        
        Self { slots: vec![vec![]; START_CAPACITY], fullness: 0.0 }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        const MAX_CAPACITY: f32 = 0.75;

        // First, if hash map if getting almost full, resize vector
        if self.fullness > MAX_CAPACITY {
            let old_slots = self.slots.clone();

            self.slots = vec![vec![]; self.slots.len() * 2]; // New vector with double the size
            self.fullness = 0.0;

            // Insert all old entries into new vector
            for slot in old_slots {
                for element in slot {
                    self.insert(element.key.as_str(), element.value);
                }
            }
        }
        
        let index = self.hash_code(key);
        
        // Increase fullness if new slot is used
        if self.slots[index].len() == 0 {
            self.fullness += 1.0 / self.slots.len() as f32;
        }

        self.slots[index].push(Element{ key: key.to_string(), value });
    }

    pub fn delete(&mut self, key: &str) {
        let index = self.hash_code(key);

        // Search for the correct element to remove
        // among potential collisions in the same slot
        for (i, element) in self.slots[index].iter().enumerate() {
            if element.key == key {
                self.slots[index].remove(i);
                return;
            }
        }
        println!("{key} does not exist");
    }

    pub fn search(&mut self, key: &str) -> Option<&mut T> {
        let index = self.hash_code(key);

        // Search for the correct element to return
        // among potential collisions in the same slot
        for (i, element) in self.slots[index].iter().enumerate() {
            if element.key == key {
                return Some(&mut self.slots[index][i].value);
            }
        }
        None // No element of the key was found
    }

    pub fn contains(&mut self, key: &str) -> bool {
        match self.search(key) {
            Some(_x) => true,
            None => false,
        }
    }

    pub fn get_elements(&self) -> Vec<Element<T>> {
        let mut elements = vec![];
        for element_slot in &self.slots {
            for element in element_slot {
                elements.push(element.clone());
            }
        }
        return elements;
    }

    fn hash_code(&self, key: &str) -> usize {
        let key_value: usize = key.as_bytes()
            .iter()
            .map(|&x| x as usize)
            .sum();
        return key_value * 17 % self.slots.len();
    }
}