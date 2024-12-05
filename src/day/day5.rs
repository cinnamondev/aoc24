
use std::{fs::File, io::Read};
use std::collections::HashMap;

pub fn run() {
    let mut file = File::open("./input/day5").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

    let strings= string.split_once("\n\n").unwrap();

    let precedence_list: HashMap<u32,Vec<u32>> = strings.0.lines()
        .fold(HashMap::new(), |mut acc, string| {
            let split: Vec<&str> = string.split('|').collect();
            acc.entry(split[1].parse::<u32>().unwrap())
                .or_default()
                .push(split[0].parse::<u32>().unwrap());
            acc
        });

    let update_string = strings.1;
    let sum: u32 = update_string.lines()
        .map(|str|  str.split(',').map(|digit| digit.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .filter(|report| update_list_is_correct(report, &precedence_list))
        .map(|valid_report| valid_report[valid_report.len()/2])
        .sum();
    println!("PART 1: {sum}");

    let sum: u32 = update_string.lines()
        .map(|str| str.split(',').map(|digit| digit.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .filter(|update| !update_list_is_correct(update, &precedence_list))
        .map(|report| { // sort list return midpoint
            let mut sorted: Vec<u32> = Vec::new();
            for page in report {
                let mut insert_at_end = true;
                if let Some(disallows) = precedence_list.get(&page) {
                    // now we need to ensure the sorted list doesnt contain anything it needs to be followed by
                    if let Some(i) = sorted.iter().position(|page| disallows.contains(page)) { // find first matching!
                        sorted.insert(i, page);
                        insert_at_end = false;
                    }
                }
                if (insert_at_end) {
                    sorted.push(page);
                }
            }
            sorted[sorted.len()/2]
        })
        .sum();
    println!("PART 2: {sum}");

}

fn update_list_is_correct(input: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, page) in input.iter().enumerate() {
        if let Some(disallowed_pages) = rules.get(&page) {
            for j in i+1..input.len() {
                // if it doesnt exist, we can ignore this anyway. so we only need to make sure its not AHEAD of us. probably not best for speed idk.
                if disallowed_pages.contains(&input[j]) { return false; }
            }
        }
    }
    true
}