use crate::object::object::Object;
use std::collections::HashMap;

pub fn abs(n: i32) -> i32 { 
    if n > 0 {
        n
    } else {
        -n
    }
}

#[derive(Clone)]

pub struct Person {
    pub taste: HashMap<u64, u8>,
}

impl Person {
    pub fn empty() -> Person {
        Person {
            taste: HashMap::new(),
        }
    }

    pub fn rate(&mut self, object_id: u64, review: u8) {
        self.taste.insert(object_id, review);
    }

    pub fn get_taste(&self, object_id: u64) -> Option<u8> {
        if let Some(ans) = self.taste.get(&object_id) {
            return Some(ans.clone());
        }
        None
    }

    pub fn simular(&self, person: &Person) -> usize {
        let mut sum: usize = 0;
        for (&key, &value) in &self.taste {
            if let Some(v) = person.taste.get(&key) {
                sum += abs(value as i32 - v.clone() as i32) as usize;
            }
        }
        sum
    }
}


