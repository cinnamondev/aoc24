use std::{fs::File, io, io::Read};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;
use std::ops::Add;
use std::thread::current;

pub fn run() {
    let mut file = File::open("./input/day10").expect("no read :(");
    let height_map: Vec<Vec<u32>> = io::BufReader::new(file).lines()
        .map(|str| {
            str.unwrap().chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    height_map.iter().for_each(|r| {
        r.iter().for_each(|c| print!("{c}"));
        println!("");
    });

    // determine which items to start from (zeroes) then traverse maze from each point
    let results: Vec<HashMap<(usize,usize), u32>> = height_map.iter().enumerate()
        .filter_map(|(i, row)| {
            let ret: Vec<(usize,usize)> = row.iter().enumerate()
                .filter_map(|(j, &height)| {
                    if height == 0 {
                        Some((j,i))
                    } else { None }
                }).collect();

            if ret.len() == 0 { None } else { Some(ret) }
        })
        .flatten()
        .map(|(x,y)| {
            let mut nines: HashMap<(usize,usize), u32> = HashMap::new();
            traverse_maze(&(x,y), &height_map, &mut nines);
            nines
        })
        .collect();

    let part1: usize = results.iter().map(|v| v.len()).sum();
    println!("Sum {part1}");

    let part2: u32 = results.iter()
        .map(|map| {
            map.iter().map(|(_,&v)| v).sum::<u32>()
        }).sum();
    println!("Rating Sum {part2}");
}

/// traverses a maze from starting position only allowing height diff of 1 between neighbours,
/// if it reaches a `9` heightmap then it's location will be added to an array.
/// will not check starting position is a 0, user should validate this.
fn traverse_maze(starting: &(usize,usize), height_map: &Vec<Vec<u32>>, nines: &mut HashMap<(usize,usize),u32>) {
    let max_x = height_map[0].len();
    let max_y = height_map.len();

    let (x,y) = starting;
    // neighbour up (y-1)
    if let Some(ny) = y.checked_sub(1) {
        let continue_exploring = process_node(starting, &(*x,ny),height_map,nines);
        if (continue_exploring) { traverse_maze(&(*x,ny), height_map, nines); }
    }

    // neighbour down
    if y+1 < max_y {
        let ny = y+1;
        let continue_exploring = process_node(starting, &(*x,ny),height_map,nines);
        if (continue_exploring) { traverse_maze(&(*x,ny), height_map, nines); }
    }

    // neighbour left (x-1)
    if let Some(nx) = x.checked_sub(1) {
        let continue_exploring = process_node(starting, &(nx,*y),height_map,nines);
        if (continue_exploring) { traverse_maze(&(nx,*y), height_map, nines); }
    }

    // neighbour right (x+1)
    if x+1 < max_x {
       let nx = x+1;
       let continue_exploring = process_node(starting, &(nx,*y),height_map,nines);
       if (continue_exploring) { traverse_maze(&(nx,*y), height_map, nines); }
    }
}

// looks at current node and says whether it's a valid candidate.
fn process_node(current_node: &(usize,usize), node: &(usize,usize), height_map: &Vec<Vec<u32>>, nines: &mut HashMap<(usize,usize),u32>) -> bool {
    let height_current = height_map[current_node.1][current_node.0];
    let height_new = height_map[node.1][node.0];
    if height_new.saturating_sub(height_current) != 1 { false } else if height_new == 9 {
        *nines.entry(*node).and_modify(|visits| *visits += 1).or_insert(1);
        false
    } else {
        true
    }
}