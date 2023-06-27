use colored::*;
use crate::collaborative_filtering::person::Person;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};

pub struct Logger {
}

impl Logger {
    pub fn info(location: String, message: String) {
        println!("[{} | {}]: {}", "INFO".bold().blue(), location.bold().yellow(), message);
    }

    pub fn error(location: String, message: String) {
        println!("[{} | {}]: {}", "ERROR".bold().red(), location.bold().yellow(), message);
    }

    pub fn result(location: String, message: String) {
        println!("\t[{} | {}]: {}", "RESULT".bold().green(), location.bold().yellow(), message);
    }

    pub fn assert(expression: bool, location: String, message: String) {
        if !expression {
            println!("[{} | {}]: {}", "ERROR".bold().red(), location.bold().yellow(), message);
        }
    }

    pub fn complete(s: &String, n: i32) -> String {
        let mut res = s.clone();
        for _i in 0..(n - (s.len() as i32)) {
            res.push_str(" ");
        }
        res.to_string()
    }

    pub fn inform_status(status: &bool) {
    println!("
-------------------------------------------------------------
|     status     | {} |
-------------------------------------------------------------



    ", Logger::complete(&status.to_string(), 40).bold().green());

    }

    pub fn execution_report(time: &String, status: &String) {
        if status == &"COMPLETE" {
            println!("



-------------------------------------------------------------
|     status     | {} |
-------------------------------------------------------------
| execution time | {} |
-------------------------------------------------------------
", Logger::complete(&status, 40).bold().green(), Logger::complete(&time, 40).bold().green());
        } else {
            println!("



-------------------------------------------------------------
|     status     | {} |
-------------------------------------------------------------
| execution time | {} |
-------------------------------------------------------------
", Logger::complete(&status, 40).bold().red(), Logger::complete(&time, 40).bold().green());
        }
    }

    pub fn show_recommendations(person: u64, recommendations: &Vec<u64>) {
        let mut table: Vec<Vec<CellStruct>> = vec![];
        let mut cur: Vec<CellStruct> = vec![person.cell()];
        for id in 0..recommendations.len() {
            cur.push((recommendations[id] + 1).cell());
        }
        table.push(cur);

        // building title
        let mut title: Vec<CellStruct> = vec!["Person".cell()];
        for i in 1..recommendations.len() + 1 {
            title.push(i.cell());
        }

        // building table
        let table = table.table().title(title).bold(true);

        // display
        let table_display = table.display().unwrap();
        println!("{}", table_display);
    }
    
    pub fn show_rankings(rankings: &Vec<(usize, usize)>) {
        println!("{{");
        for id in 0..rankings.len() {
            println!("Ranking {}: ({}, {})", id + 1, rankings[id].0, rankings[id].1 + 1);
        }
        println!("}}");
    }

    pub fn show_people(people: &Vec<Person>, shows: usize) {
        let mut table: Vec<Vec<CellStruct>> = vec![];
        for id in 0..people.len() {
            let mut cur: Vec<CellStruct> = vec![];
            cur.push((id + 1).cell());
            for movie in 0..shows {
                if let Some(review) = people[id].get_taste(movie as u64) {
                    cur.push(review.cell());
                } else {
                    cur.push((-1).cell());
                }
            }
            table.push(cur);
        }
        // building title
        let mut title: Vec<CellStruct> = vec!["Person".cell()];
        for i in 1..6 {
            title.push(i.cell());
        }

        // building table
        let table = table.table().title(title).bold(true);

        // display
        let table_display = table.display().unwrap();
        println!("{}", table_display);
    }
}













