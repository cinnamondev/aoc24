use std::{fs::File, io::Read};

use regex::Regex;

pub fn run() {
    let mut file = File::open("./input/day3").expect("no read :(");
    let mut string = String::new();;
    file.read_to_string(&mut string).expect("waaah");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let matches: u32 = re.captures_iter(&string)
    .map(|captures| {
        let l = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let r = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        println!("l: {} r: {}", l,r);
        l*r
    }).sum();


    println!("{}",matches);
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut parse_muls = true;
    for captures in re.captures_iter(&string) {
        let instruction = captures.get(0).unwrap().as_str();
        if (instruction == "do()") {
            parse_muls = true;
            continue;
        } else if (instruction == "don't()") {
            parse_muls = false;
            continue;
        }

        if (parse_muls) {
            let l = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let r = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            println!("l: {} r: {}", l,r);
            sum += l*r;
        }
    }

    println!("{}",sum);

}
