use std::str::FromStr;
use aocrs::input::get_lines;
use crate::page_order::PageOrderMap;
use crate::update::Update;

mod page_order;
mod update;

fn get_input(filename: &str) -> (PageOrderMap, Vec<Update>) {
    let lines = get_lines(filename).unwrap();
    let mut page_order = PageOrderMap::default();
    let mut updates: Vec<Update> = Vec::with_capacity(500);
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        }

        if line.as_bytes()[2] == b'|' {
            page_order.add(&line);
        } else {
            updates.push(Update::from_str(&line).unwrap());
        }
    }
    (page_order, updates)
}

fn main() {
    let (page_order, updates) = get_input("./day05/input.txt");
    let valid_sum = &updates
        .iter()
        .filter(|x| x.is_valid(&page_order))
        .map(|x| x.middle())
        .sum::<i32>();
    println!("part 1: {}", valid_sum);      // 143

    let invalid_sum = &updates
        .iter()
        .filter(|x| !x.is_valid(&page_order))
        .map(|x| x.fix(&page_order).middle())
        .sum::<i32>();
    println!("part 2: {}", invalid_sum);    // 123
}
