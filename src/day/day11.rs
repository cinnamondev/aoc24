use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::thread;
use std::thread::{JoinHandle, ScopedJoinHandle};
use std::time::Instant;

pub fn run() {
    let mut file = File::open("./input/day11").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

    let before = Instant::now();
    let (part1,part2) = parse_line(&string);
    println!("Single threaded time: {:.2?}", before.elapsed());
    println!(",1 {}", part1);
    println!(",2 {}", part2);

    let before = Instant::now();
    let (part1,part2) = parse_line_multithread(&string);
    println!("Multi threaded time: {:.2?}", before.elapsed());
    println!(",1 {}", part1);
    println!(",2 {}", part2);
}
fn parse_line_multithread(line: &String) -> (u64,u64) {
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    thread::scope(|s| {
        let strings: Vec<&str> = line.split_whitespace().collect();
        let mut handles: Vec<ScopedJoinHandle<(u64,u64)>> = Vec::new();
        for string in strings {
            handles.push(
                s.spawn(move || {
                    let mut stone_map: HashMap<u64,u64> = HashMap::new();
                    stone_map.entry(string.parse::<u64>().unwrap()).or_insert(1);
                    let mut p1_sum = 0;
                    for i in 0..75 {
                        if i == 25 { p1_sum += stone_map.values().sum::<u64>(); }

                        let mut to_parse_next: HashMap<u64,u64> = HashMap::new();
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
                    (p1_sum, stone_map.values().sum::<u64>())
                })
            );
        }
        for handle in handles {
            let (sum_rock_1,sum_rock_2) = handle.join().expect("uh ohhh");
            p1_sum += sum_rock_1; p2_sum += sum_rock_2;
        }
    });
    (p1_sum, p2_sum)
}

fn parse_line(line: &String) -> (u64,u64) {
    line.split_whitespace().enumerate().map(|(n,string_stone)| {
        let mut stone_map: HashMap<u64,u64> = HashMap::new();
        stone_map.entry(string_stone.parse::<u64>().unwrap()).or_insert(1);

        let mut part1_sum = 0;
        for i in 0..75 {
            if i == 25 { part1_sum = stone_map.values().sum(); }

            let mut to_parse_next: HashMap<u64,u64> = HashMap::new();
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
        (part1_sum, stone_map.values().sum::<u64>())
    }).fold((0,0), |(mut part1, mut part2), (new_1,new_2)| {
        part1 += new_1;
        part2 += new_2;
        (part1,part2)
    })
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