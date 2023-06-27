use crate::{collaborative_filtering::person::Person, logger::logger::Logger};
use std::collections::HashMap;
use crate::object::object::Object;

const LIMIT: usize = 100;

pub fn ranking(review: &u8, relevance: &u64) -> usize {
    let rel: usize = relevance.clone() as usize;
    let rev: usize = review.clone() as usize;
    if rel <= LIMIT {
        return rev * (LIMIT - rel);
    }
    0
}

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

    pub fn slow_recommend(&mut self, person_id: u64, feed: i32) -> Vec<u64> {
        // Time Complexity: 
        let mut people: Vec<(usize, u64)> = vec![];
        for id in 0..self.people.len() {
            if id == person_id as usize { continue; }

            let simularity: usize = self.people[person_id as usize].simular(&self.people[id]);

            println!("{} person is simular to {}: {}", person_id + 1, id + 1, simularity);

            people.push(
                (simularity, id as u64)
            );
        }
        people.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        Logger::info("src/collaborative_filtering/slow_recommend".to_string(), "Sorted people by simularity".to_string());

        let mut object_ranks: Vec<(usize, usize)> = vec![(0, 0); self.products.len()];
        for id in 0..people.len() {
            let products_ids: Vec<_> = self.people[people[id].1 as usize].taste.keys().clone().collect();
            for product_id in products_ids {
                if let Some(taste) = self.people[people[id].1 as usize].get_taste(product_id.clone()) {
                    object_ranks[product_id.clone() as usize] =
                        (object_ranks[product_id.clone() as usize].0 + ranking(&taste, &(id.clone() as u64)) as usize,
                        product_id.clone() as usize);
                }
            }
        }
        object_ranks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        Logger::show_rankings(&object_ranks);
        let mut recommendation: Vec<u64> = vec![];
        for i in 0..feed {
            recommendation.push(object_ranks[i as usize].1 as u64);
        }
        recommendation
    }
}











