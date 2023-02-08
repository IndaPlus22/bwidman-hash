#[derive(Clone)]
pub struct Element<T>
where T: Copy {
    key: String,
    value: T
}

pub struct HashMap<T>
where T: Copy {
    slots: Vec<Vec<Element<T>>>
}

impl<T> HashMap<T>
where T: Copy {
    pub fn new() -> Self {
        const START_CAPACITY: usize = 32;
        
        Self { slots: vec![vec![]; START_CAPACITY] }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        const MAX_CAPACITY: f32 = 0.75;

        // First, if hash map if getting almost full (ratio of used slots > MAX_CAPACITY)
        // resize vector
        
        
        let index = self.hash_code(key);
        self.slots[index].push(Element{ key: key.to_string(), value });
    }

    pub fn delete(&mut self, key: &str) {
        let index = self.hash_code(key);

        // Search for the correct element to remove among
        // potential collisions in the same hash map slot
        for (i, element) in self.slots[index].iter().enumerate() {
            if element.key == key {
                self.slots[index].remove(i);
                return;
            }
        }
    }

    pub fn search(&mut self, key: &str) -> Option<T> {
        let index = self.hash_code(key);

        // Search for the correct element to return among
        // potential collisions in the same hash map slot
        for (i, element) in self.slots[index].iter().enumerate() {
            if element.key == key {
                return Some(self.slots[index][i].value);
            }
        }
        None // No element of the key was found
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
            .map(|x| *x as usize)
            .sum();
        return key_value * 17 % self.slots.len();
    }
}