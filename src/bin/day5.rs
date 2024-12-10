use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};
fn main() {
    prob2();
}
fn prob2() {
    let file = File::open("data/day5.txt").expect("File not found");

    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.expect("Couldnt read the file?"));
    //.collect::<Vec<String>>();
    let mut rules: Vec<(String, String)> = Vec::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut rule_end = false;

    for line in lines {
        if line.trim().is_empty() {
            rule_end = true;
        } else if rule_end {
            let split: Vec<String> = line.split(",").map(|f| f.to_string()).collect();
            updates.push(split);
        } else {
            let mut split = line
                .split("|")
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let after = split.pop().unwrap();
            let before = split.pop().unwrap();
            rules.push((before, after));
        }
    }

    println!(" rules {:?}", rules);
    println!(" updates {:?}", updates);

    let mut result = 0;

    for update in updates {
        let ordered = check_update(&rules, &update);

        if !ordered {
            let mut ordered_update = update;

            order_update(&rules, &mut ordered_update);
            let page_no = &ordered_update[(ordered_update.len() - 1) / 2]
                .parse::<i32>()
                .unwrap();
            result += page_no
        }
    }

    println!(" result {:?}", result);
}

fn order_update(rules: &Vec<(String, String)>, update: &mut Vec<String>) {
    // if !check_update(rules, update)
    // {
    let swap = update
        .iter()
        .enumerate()
        .find_map(|(index, page_no)| check_page_index(index, page_no, rules, update));
    if swap.is_some() {
        swap.iter().for_each(|(i1, i2)| update.swap(*i1, *i2));
        order_update(rules, update);
    }
}

fn check_update(rules: &Vec<(String, String)>, update: &Vec<String>) -> bool {
    update
        .iter()
        .enumerate()
        .all(|(index, page_no)| check_page(index, page_no, &rules, &update))
}

fn check_page(
    index: usize,
    page_no: &str,
    rules: &Vec<(String, String)>,
    update: &Vec<String>,
) -> bool {
    rules
        .iter()
        .filter(|(before, _)| before == page_no)
        .all(|(_, after)| check_rule(index, after, update))
}

fn check_rule(index: usize, after: &str, update: &Vec<String>) -> bool {
    update[0..index].iter().all(|page_no| page_no != after)
}
fn check_rule_index(index: usize, after: &str, update: &Vec<String>) -> Option<usize> {
    update[0..index]
        .iter()
        .enumerate()
        .find(|(_, page_no)| *page_no == after)
        .map(|(index, _)| index)
}

fn check_page_index(
    index: usize,
    page_no: &str,
    rules: &Vec<(String, String)>,
    update: &Vec<String>,
) -> Option<(usize, usize)> {
    rules
        .iter()
        .filter(|(before, _)| before == page_no) // Is rule relevant
        .find_map(|(_, after)| check_rule_index(index, after, update)) // find first break
        .map(|found| (found, index)) // create swap
}

fn prob1() {
    let file = File::open("data/day5.txt").expect("File not found");

    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.expect("Couldnt read the file?"));
    //.collect::<Vec<String>>();

    let mut rules: Vec<(String, String)> = Vec::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    let mut rule_end = false;

    for line in lines {
        if line.trim().is_empty() {
            rule_end = true;
        } else if rule_end {
            let split: Vec<String> = line.split(",").map(|f| f.to_string()).collect();
            updates.push(split);
        } else {
            let mut split = line
                .split("|")
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let after = split.pop().unwrap();
            let before = split.pop().unwrap();
            rules.push((before, after));
        }
    }

    println!(" rules {:?}", rules);
    println!(" updates {:?}", updates);

    let mut result = 0;

    for update in updates {
        let ordered = update
            .iter()
            .enumerate()
            .all(|(index, page_no)| check_page(index, page_no, &rules, &update));

        if ordered {
            let page_no = &update[(update.len() - 1) / 2].parse::<i32>().unwrap();
            result += page_no
        }
    }

    println!(" result {:?}", result);
}
