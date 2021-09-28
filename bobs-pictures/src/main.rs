use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use time::{format_description, Date};

const INPUT_STRING: &str = "A.png, Barcelona, 2012-08-11
B.png, Madrid, 2014-09-09
C.jpg, Barcelona, 2012-08-09";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Name(String, String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct City(String);

#[derive(Clone, Debug)]
struct File {
    name: Name,
    city: City,
    date: Date,
}

fn main() {
    let format = format_description::parse("[year]-[month]-[day]").unwrap();

    // Parse files
    let files = INPUT_STRING
        .lines()
        .map(|ln| ln.split(", ").collect::<Vec<_>>())
        .map(|ln| {
            let name = ln[0].split('.').collect::<Vec<_>>();
            File {
                name: Name(name[0].to_string(), name[1].to_string()),
                city: City(ln[1].to_string()),
                date: Date::parse(ln[2], &format).unwrap(),
            }
        })
        .collect::<Vec<_>>();

    // Mapping of all cities
    let mut mapping: HashMap<City, BinaryHeap<Date>> = HashMap::new();

    // Read all files to determine their order
    for file in &files {
        let ct = mapping.entry(file.city.clone()).or_default();
        ct.push(file.date);
    }

    // Use mapping to generate new file names
    files.into_iter().for_each(|fil| {
        let n = mapping
            .get(&fil.city)
            .unwrap()
            .iter()
            .rev()
            .position(|dt| dt == &fil.date)
            .unwrap()
            + 1;

        println!("{}{}.{}", fil.city.0, n, fil.name.1);
    });
}
