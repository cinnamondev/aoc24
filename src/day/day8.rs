use core::fmt;
use std::arch::x86_64::_CMP_FALSE_OQ;
use std::fmt::Write;
use std::hash::Hash;
use std::io::BufRead;
use std::ops::Sub;
use std::{io, result};
use std::{fs::File, io::Read};
use std::collections::HashMap;

// its starting to look like day 6

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Antenna {
    symbol: char,
    x: usize,
    y: usize,
}

impl fmt::Display for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.symbol)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NodeType {
    Antenna(Antenna),
    Free
}


#[derive(Debug, Clone)]
struct Node {
    typ: NodeType,
    antinodes: Vec<Antenna>
}

impl Node {
    pub fn new(node_type: NodeType) -> Node {
        Node {
            typ: node_type,
            antinodes: Vec::new()
        }
    }
}

// this is for pretty grid
impl fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let antinodes_present = self.antinodes.len() > 0;
        let str = if let NodeType::Antenna(a) = &self.typ {
            if antinodes_present {
                "\x1b[41m".to_owned() + &a.symbol.to_string() + "\x1b[0m"
            } else {
                a.symbol.to_string()
            }
        } else {
            if antinodes_present {
                "#".to_string()
            } else {
                ".".to_string()
            }
        };
        f.write_str(str.as_str())
    }
}

pub fn run() {
    let mut file = File::open("./input/day8").expect("no read :(");
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    let grid: Vec<Vec<Node>> = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(i,line)| {
            let line = line.unwrap();
            line.chars().enumerate().map(|(j,c)| {
                match c {
                    '.' => Node::new(NodeType::Free),
                    antenna_symb=> {
                        let antenna = Antenna {
                            symbol: antenna_symb,
                            x: j,
                            y: i,
                        };
                        antennas.entry(antenna_symb).or_default().push(antenna);
                        Node::new(NodeType::Antenna(antenna))
                    }
                }
            }).collect::<Vec<Node>>()
        }).collect();

    let mut grid_1 = grid.clone();
    solve(&antennas, &mut grid_1, false);

    let positions: usize = grid_1.iter().map(|l| {
        println!("");
        l.iter().filter(|n| {print!("{n}"); n.antinodes.len() > 0}).collect::<Vec<_>>().len()
    }).sum();
    println!("\n{positions}");

    let mut grid_2 = grid.clone();
    solve(&antennas, &mut grid_2, true);

    let positions: usize = grid_2.iter().map(|l| {
        println!("");
        l.iter().filter(|n| {print!("{n}"); n.antinodes.len() > 0}).collect::<Vec<_>>().len()
    }).sum();
    println!("\n{positions}");
}

fn solve(antennas: &HashMap<char,Vec<Antenna>>, grid: &mut Vec<Vec<Node>>, resonant: bool) {
    
    for (k,v) in antennas.iter() {
        for (i,ant) in v.iter().enumerate() {
            for j in 0..v.len() {
                if j==i {
                    if resonant && v.len() != 1 { // if resonant mode and multiple antennas then mark resonant too
                        grid[ant.y][ant.x].antinodes.push(*ant); 
                    }
                    continue;
                } // exclude self as will mark self otherwise.

                let dx = ant.x as isize - v[j].x as isize;
                let dy = ant.y as isize - v[j].y as isize;

                let mut prev_x = ant.x; let mut prev_y = ant.y;
                // will propogate to edges of grid if resonant
                while let (Some(x),Some(y)) = (prev_x.checked_add_signed(dx), prev_y.checked_add_signed(dy)) {
                    if x < grid[0].len() && y < grid.len() {
                        grid[y][x].antinodes.push(*ant);
                        prev_x = x;
                        prev_y = y;
                    } else { break; }
                    if !resonant { break; } // only exec once if not resonant mode.
                }
                // if not true, we can safely ignore as it is out of bounds.

            }
        }
    };
    
}