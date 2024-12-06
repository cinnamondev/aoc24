mod day;

use day::day1;
use day::day2;
use day::day3;
use day::day4;
use day::day5;
use day::day6;

fn main() {
    println!("output day 1");
    day1::run();

    println!("output day2");
    day2::run();

    println!("output day3");
    day3::run();

    println!("day44444");
    day4::run();
    
    println!("day5");
    day5::run();

    println!("day6");
    day6::run(false);

    println!("Hello, world!");
}
