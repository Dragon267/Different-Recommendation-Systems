use crate::{
    person::person::Person,
    logger::logger::Logger
};

use std::collections::HashMap;
use crate::object::object::Object;

pub struct System {
    pub products: Vec<Object>,
    pub people: Vec<Person>,
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

    pub fn recommend(&mut self, person_id: u64, feed: i32, N: usize) -> Vec<u64> {
        // Time Complexity: O(n + n log n + n^2 + n log n) ~= O(n^2 + n log n)
        let mut people: Vec<(f32, u64)> = vec![];
        for id in 0..self.people.len() {
            if id == person_id as usize { continue; }

            let simularity: f32 = self.people[person_id as usize].simular(&self.people[id]);

            println!("{} person is simular to {}: {}", person_id + 1, id + 1, simularity);

            people.push(
                (simularity, id as u64)
            );
        }
        people.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        Logger::info("src/collaborative_filtering/slow_recommend".to_string(), "Sorted people by simularity".to_string());

        let mut object_ranks: Vec<(f32, usize)> = vec![(-1.0, 0); self.products.len()];
        for id in 0..object_ranks.len() {
            let mut sum: f32 = 0.0;
            for best_index in 0..(N as usize) {
                sum += self.people[people[best_index].1 as usize]
                    .get_taste(id as u64) as f32;
            }
            sum = sum * (1.0 / (N as f32));

            object_ranks[id] = (sum, id);
        }

        object_ranks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let mut recommendation: Vec<u64> = vec![];
        let mut count: usize = feed as usize;
        for i in 0..object_ranks.len() {
            if count == 0 {
                break;
            }
            if self.people[person_id as usize].get_taste(i as u64) != 0 {
                continue;
            }
            println!("RECOMMENDATION: {} {}", i, self.people[person_id as usize].get_taste(i as u64));
            recommendation.push(i as u64);
            count -= 1;
        }
        recommendation
    }
}











