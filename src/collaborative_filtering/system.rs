use crate::collaborative_filtering::person::Person;
use std::collections::HashMap;
use crate::object::object::Object;

pub fn f1(x: f32)  {
}

pub fn ranking(review: &u8, relevance: &u64) -> f32 {
}

pub struct System {
    products: Vec<Object>,
    people: Vec<Person>,
}

impl System {
    pub fn rate(&mut self, person_id: u64, object_id: u64, review: u8) { // this function rates a Object
        self.people[person_id as usize].rate(object_id, review);
    }

    pub fn add_product(&mut self, product: Object) {
        self.products.push(product);
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }

    pub fn slow_recommend(&mut self, person_id: u64, feed: i32) -> Vec<u64> {
        let mut people: Vec<(f32, u64)> = vec![];
        for id in 0..self.people.len() {
            let simularity: f32 = self.people[person_id as usize].simular(&self.people[id]);
            people.push(
                (simularity, id as u64)
            );
        }
        people.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        vec![]
    }
}

