use crate::object::object::Object;
use std::collections::HashMap;

pub fn abs(n: f32) -> f32 {
    if n > 0.0 {
        n
    } else {
        -n
    }
}

pub struct Person {
    pub taste: HashMap<u64, u8>,
}

impl Person {
    pub fn rate(&mut self, object_id: u64, review: u8) {
        self.taste.insert(object_id, review);
    }

    pub fn simular(&self, person: &Person) -> f32 {
        let mut sum: f32 = 0.0;
        for (&key, &value) in &self.taste {
            if let Some(v) = person.taste.get(&key) {
                sum += abs((value - v) as f32);
            }
        }
        sum
    }
}


