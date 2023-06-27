use crate::collaborative_filtering::person::Person;
use crate::collaborative_filtering::system::System;
use crate::object::object::Object;
use crate::logger::logger::Logger;

use rand::Rng;

fn random_number(l: i32, r: i32) -> i32 {
    rand::thread_rng().gen_range(l, r + 1)
}

fn random_rating() -> u8 {
    // random number from 1 to 10
    rand::thread_rng().gen_range(1, 11) as u8 
}

pub fn status() -> bool {
    true
}

pub fn test() {
    let person: u64 = 0;

    let objects: Vec<Object> = vec![
        Object::just_name(
            "Movie1",
        ),
        Object::just_name(
            "Movie2",
        ),
        Object::just_name(
            "Movie3",
        ),
        Object::just_name(
            "Movie4",
        ),
        Object::just_name(
            "Movie5",
        ),
    ];
    let mut people: Vec<Person> = vec![Person::empty(); 5];
    for i in 0..people.len() {
        for j in 0..3 {
            people[i].rate(random_number(0, (objects.len() - 1) as i32) as u64, random_rating());
        }
    }
    Logger::show_people(&people, 5);

    let mut system: System = System {
        products: objects,
        people
    };

    Logger::info("src/test/test.rs/test".to_string(), "Building recommendations...".to_string());
    let recommendations: Vec<u64> = system.slow_recommend(person.clone(), 3);
    Logger::info("src/test/test.rs/test".to_string(), "Recommendations built!".to_string());

    Logger::show_recommendations(person, &recommendations);
}













