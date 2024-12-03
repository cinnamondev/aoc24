use std::{fs::File, io::Read};

pub fn run() {
    let mut file = File::open("./input/day1").expect("no read :(");
    let mut str = String::new();

    file.read_to_string(&mut str).unwrap();

    let mut l: Vec<u32> = str.split_whitespace()
        .step_by(2)
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    l.sort();

    let mut r: Vec<u32> = str.split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    r.sort();

    let similarity: u32 = l.iter()
        .map(|&n| n * (r.iter()
                                .filter(|&&n_r| n_r == n)
                                .collect::<Vec<&u32>>().len() as u32)
        ).sum();

    println!("SIMILARITY SCORE {}", similarity);

    let mut tot_distance: u32 = 0;
    for (i, &it_r) in r.iter().enumerate() {
        let diff = if (it_r > l[i]) {it_r - l[i]} else {l[i] - it_r};
        tot_distance += diff;
    }
    println!("TOTAL DISTANCE {}", tot_distance)
}