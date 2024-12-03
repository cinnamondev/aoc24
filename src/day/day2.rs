use std::{fs::File, io::{self, BufRead, Read}, iter};

pub fn run() {
    let file = File::open("./input/day2").expect("no read :(");

    let reports: Vec<bool> = io::BufReader::new(file).lines()
        .map(|_line| {
            let line: String = _line.expect("read error :(");
            parse_line(&line)
        }).collect();

    let safe_reports = reports.iter().filter(|&&safe| safe).collect::<Vec<&bool>>().len();
    println!(" safe : {}", safe_reports);

    let file = File::open("./input/day2").expect("no read :(");
    // part 2 remember unsafe once!
    let reports: Vec<bool> = io::BufReader::new(file).lines()
        .map(|_line| {
            let line: String = _line.expect("read error :(");
            if (!parse_line(&line)) {
                for (i, digits) in line.split_whitespace().enumerate() {
                    let mut copy_line: Vec<&str> = line.split_whitespace().collect();
                    copy_line.remove(i);
                    if parse_line(&copy_line.join(" ")) {
                        return true;
                    }
                }
                //println!("unsafe regardless of removal");
                return false;
            } else {
                return true;
            }
        }).collect();

    //println!("{:?}", reports);
    let safe_reports = reports.iter().filter(|&&safe| safe).collect::<Vec<&bool>>().len();
    println!(" safe : {}", safe_reports);
}

fn parse_line(line: &str) -> bool {
    let mut prev = 0;
    let mut prev_increasing = true;
    //print!("{}",line);
    for (i,n) in line.split_whitespace().map(|d| d.parse::<u32>().unwrap()).enumerate() {
        if (i == 0) { prev=n; } else {
            let increasing = n > prev;
            if (i!=1 && increasing != prev_increasing) {
                //println!("   ... UNSAFE (unstable gradient)");
                return false;
            }
            prev_increasing = increasing;
            let diff = if (increasing) { n - prev } else { prev - n };
            if ((diff < 1) || (diff > 3)) { 
                //println!("   ... UNSAFE (unstable difference)");
                return false; 
            }
            prev = n;   
        }
    }
    //println!("   ... SAFE!");
    return true;
}