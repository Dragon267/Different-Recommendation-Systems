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
    pub taste: Vec<u8>,
}

impl Person {
    pub fn empty(objects: usize) -> Person {
        Person {
            taste: vec![0; objects],
        }
    }

    pub fn rate(&mut self, object_id: u64, review: u8) {
        self.taste[object_id as usize] = review;
    }

    pub fn get_taste(&self, object_id: u64) -> u8 {
        return self.taste[object_id as usize];
    }

    pub fn simular(&self, person: &Person) -> f32 {
        let mut up: usize = 0;
        for i in 0..self.taste.len() {
            up += (self.taste[i] as usize) * (person.get_taste(i as u64) as usize);
        }

        let mut down_1: f32 = 0.0;
        for i in 0..self.taste.len() {
            down_1 += ((self.taste[i] as usize) * (self.taste[i] as usize)) as f32;
        }

        let mut down_2: f32 = 0.0;
        for i in 0..self.taste.len() {
            down_2 += ((person.get_taste(i as u64) as usize) * (person.get_taste(i as u64) as usize)) as f32;
        }

        let down: f32 = (down_1 * down_2).sqrt();

        (up as f32) / down
    }
}


