use core::fmt;
use std::char::from_digit;
use std::fmt::Write;
use std::hash::Hash;
use std::io::BufRead;
use std::ops::Sub;
use std::path::Display;
use std::{io, result};
use std::{fs::File, io::Read};
use std::collections::HashMap;

#[derive(Clone, Copy,Debug, PartialEq, Eq)]
enum Block {
    File(u32),
    Free,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Block::File(id) => f.write_str(&id.to_string()),
            Block::Free => f.write_char('.'),
        }
    }
}

pub fn run() {
    let mut file = File::open("./input/day9").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

    let mut id = 0;
    let mut linear_fs: Vec<Block> = Vec::new();
    for (n,c) in string.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        if n % 2 != 0 {
            let mut blocks = vec![Block::Free; c.to_digit(10).unwrap().try_into().unwrap()];
            linear_fs.append(&mut blocks);
        } else {
            let r = Block::File(id);
            let mut blocks = vec![r; c.to_digit(10).unwrap().try_into().unwrap()];
            id+=1;
            linear_fs.append(&mut blocks);
        }
    }
    println!("{}",linear_fs.len());
    solve(&mut linear_fs);
    println!("{}", checksum(&linear_fs));
    print_fs(&linear_fs);
}

fn solve(linear_fs: &mut Vec<Block>) {
    let mut tot = 0;
    for i in 0..linear_fs.len() {
        if linear_fs[i] == Block::Free {
            let opt = linear_fs.iter().enumerate()
                .skip(i)
                .filter(|(_,block)| if let Block::File(_) = block { true } else { false })
                .last();
            if let Some((i_last, block)) = opt {
                linear_fs[i] = *block;
                linear_fs[i_last] = Block::Free;
            } else { break; } // everything ahead is sorted (no more blocks ahead of fs head)
        }
    }
}

fn checksum(linear_fs: &Vec<Block>) -> u64 {
    linear_fs.iter().enumerate().filter_map(|(i,block)| {
        if let Block::File(id) = block {
            Some(i as u64 * (*id as u64))
        } else {
            None
        }
    }).sum()
}

fn print_fs(linear_fs: &Vec<Block>) {
    linear_fs.iter().for_each(|c| print!("{c}"));
    println!("");
}