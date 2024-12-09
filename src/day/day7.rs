use std::arch::x86_64::_CMP_FALSE_OQ;
use std::result;
use std::{fs::File, io::Read};
use std::collections::HashMap;

enum Operator {
    ADD,
    MUL,
}
pub fn run() {
    let mut file = File::open("./input/day7").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

// numbers getitng too big for u32 oop
    let input: Vec<(u64, Vec<u64>)> = string.lines()
        .map(|line| {
            println!("{line}");
            let keyHash = line.split_once(":").unwrap();
            (keyHash.0.parse::<u64>().unwrap(), keyHash.1.split_whitespace().map(|digit| digit.parse::<u64>().unwrap()).collect())
        })
        .collect();

    let p1: u64 = input.iter().filter(|(total, values)| check_line(*total, values,false)).map(|(t,_)| t).sum();
    let p2: u64 = input.iter().filter(|(total, values)| check_line(*total, values,true)).map(|(t,_)| t).sum();
    println!("{p1}");
    println!("{p2}");
}

pub fn check_line(total: u64, values: &[u64], concat_operator: bool) -> bool {
    if (values.len() == 0) { return total == 0 } // there are no more values left (exit condition)
    let (l,values) = values.split_last().unwrap();
    // check if div or sub bring us closer to 0, it doenst matter what combination of things so we just check whatever until the list is emptuy
    ((total % *l == 0) && check_line(total/l, values, concat_operator)) 
        || ((total >= *l) && check_line(total-l, values, concat_operator))
        || if (concat_operator) { // lhs || rhs but in reverse, check if a total value has the potential to be split up then check if the next value is a candidate (/1000 ~=0,/100~=0,...)
            println!("reach {total}");
            if *l < 10 {
                (total % 10 == *l) && check_line(total/10, values, concat_operator)
            } else if *l < 100 {
                (total % 100 == *l) && check_line(total/100, values, concat_operator)
            } else if *l < 1000 {
                println!("{}",total%1000);
                (total % 1000 == *l) && check_line(total/1000, values, concat_operator)
            } else {
                // error?
                println!("line num cannot be larger than 3 digits so do not qualify");
                false
            }
        } else { false }
}
