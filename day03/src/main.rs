use aocrs::input::get_all_lines;
use regex::Regex;

fn get_mul_sum(re: &Regex, input: &str) -> i32 {
    let mut result = 0;
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        result += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }
    result
}

fn get_conditional_mul_sum(re: &Regex, input: &str) -> i32 {
    let do_idxes: Vec<_> = input.match_indices("do()").map(|(idx, _)| idx).collect();
    let dont_idxes: Vec<_> = input.match_indices("don't()").map(|(idx, _)| idx).collect();
    let mut result = 0;
    for cap in re.captures_iter(input) {
        let (_, [first, second]) = cap.extract();
        let pos = cap.get(0).unwrap().start();

        let do_idx = do_idxes.iter().filter(|&x| x < &pos).max();
        let dont_idx = dont_idxes.iter().filter(|&x| x < &pos).max();
        if do_idx.is_none() && dont_idx.is_some() {
            // we haven't encountered any do() but has a don't()
            continue;
        }
        if do_idx.is_some() && dont_idx.is_some() && do_idx.unwrap() < dont_idx.unwrap() {
            // when a do() appears before a don't()
            continue;
        }

        result += first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
    }
    result
}

fn main() {
    let input = get_all_lines("./day03/input.txt").unwrap().join("");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    println!("part 1: {}", get_mul_sum(&re, &input));                   // 161
    println!("part 2: {}", get_conditional_mul_sum(&re, &input));       // 48
}
