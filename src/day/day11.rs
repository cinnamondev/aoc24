use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};

pub fn run() {
    let mut file = File::open("./input/day11").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

    let part1 = parse_line(&string, 25);
    println!(",1 {}", part1);

    let part2 = parse_line(&string, 75);
    println!(",2 {}", part2);
}

fn parse_line(line: &String, blinks: usize) -> usize {
    line.split_whitespace().enumerate().map(|(n,string_stone)| {
        let mut stone_map: HashMap<u64,usize> = HashMap::new();
        stone_map.entry(string_stone.parse::<u64>().unwrap()).or_insert(1);

        for i in 0..blinks {
            let mut to_parse_next: HashMap<u64,usize> = HashMap::new();
            //println!("stone {n} , pass {}",i+1);
            for (rock,count) in stone_map {
                let (original, opt_rock) = next_state(rock);
                to_parse_next.entry(original)
                    .and_modify(|c| *c+=count )
                    .or_insert(count);
                if let Some(new_rock) = opt_rock {
                    to_parse_next.entry(new_rock)
                        .and_modify(|c| *c+=count )
                        .or_insert(count);
                }
            }
            stone_map = to_parse_next;
        }
        stone_map.values().sum::<usize>()
    }).sum()
}

fn next_state(rock_value: u64) -> (u64, Option<u64>) {
    let n_digits = rock_value.checked_ilog10().unwrap_or(0) + 1;
    //print!("{} ", rock_value);
    if rock_value == 0 {
        (1, None)
    } else if n_digits % 2 == 0 { // even number of digits
        let divisor = 10_u64.pow(n_digits/2);
        let new_value = rock_value / divisor;
        let ret = Some(rock_value - (new_value * divisor));
        (new_value, ret)
    } else {
        // if all other rules fail.
        (rock_value*2024,None)
    }
}