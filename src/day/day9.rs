use core::fmt;
use std::char::from_digit;
use std::fmt::{Formatter, Write};
use std::hash::Hash;
use std::io::BufRead;
use std::ops::Sub;
use std::path::Display;
use std::{io, result};
use std::{fs::File, io::Read};
use std::arch::x86_64::_mm256_lddqu_si256;
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::day::day9::BlockType::Free;

#[derive(Clone, Copy,Debug, PartialEq, Eq)]
enum BlockType {
    File(u32), // size, id
    Free,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Block {
    typ: BlockType,
    quantity: u32,
}

impl Block  {
    /// ignoring the type of the resident block, will fill current block
    /// with new block. returns a block to insert following the current block
    /// if the occupying block does not fill the old block.
    /// return .0 : excess free space block .1 : whether .0 is after self or other.
    pub fn fill_from(&mut self, other: &mut Block) -> (Option<Block>, bool) {
        let mut self_free_quantity = 0;
        let mut other_free_quantity = 0;
        self.typ = other.typ;
        if (self.quantity >= other.quantity) {
            // self can entirely consume other
            self_free_quantity = self.quantity - other.quantity; // the excess
            self.quantity = other.quantity;
            other.typ = BlockType::Free;
        } else {
            // self is entirely consumed by other
            other_free_quantity = other.quantity - self.quantity; // the blocks freed on other
            // self.quantity wont change
            other.quantity = other.quantity - self.quantity;

        }

        (
            if self_free_quantity > 0 {
                Some(Block {
                    typ: BlockType::Free,
                    quantity: self_free_quantity
                })
            } else if other_free_quantity > 0 {
                Some(Block {
                    typ: BlockType::Free,
                    quantity: other_free_quantity
                })
            } else { None },
            self_free_quantity > 0
        )

    }
}
impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.typ {
            BlockType::File(id)=> f.write_str(&id.to_string().repeat(self.quantity as usize)),
            BlockType::Free => f.write_str(&".".repeat(self.quantity as usize)),
        }
    }
}

pub fn run() {
    let mut file = File::open("./input/day9").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

    let mut id = 0;
    let mut linear_fs: Vec<Block> = string.chars().enumerate()
        .map(|(i,c)| {
            let n_blocks = c.to_digit(10).unwrap(); // wont exceed usize bound.
            let mut block_type = if i % 2 == 0 {
                id+=1;
                BlockType::File(id-1)
            } else {
                BlockType::Free
            };

            Block {
                typ: block_type,
                quantity: n_blocks
            }
    }).collect();

    let mut fs1 = linear_fs.clone();
    solve(&mut fs1);
    //print_fs(&fs1);
    println!("{}",checksum(&fs1));

    let mut fs2 = linear_fs.clone();
    solve_abs(&mut fs2);
    //print_fs(&fs2);
    println!("{}",checksum(&fs2));
}

fn solve(linear_fs: &mut Vec<Block>) {
    let mut first_empty = linear_fs.iter().enumerate().find(|(i,block)| block.typ == BlockType::Free);
    let mut last_digit =  linear_fs.iter().enumerate().rev().find(|(i,block)| block.typ != BlockType::Free);
    while let (Some((i_f, _)), Some((i_l, bucket))) = (first_empty, last_digit){
        if i_f > i_l { break; }

        let mut new_bucket = bucket.clone();
        let (insert, ahead_of_current) = linear_fs[i_f]
            .fill_from(&mut new_bucket);
        linear_fs[i_l] = new_bucket;

        if let Some(insert_free) = insert {
            if (ahead_of_current) {
                linear_fs.insert(i_f+1, insert_free);
            } else {
                linear_fs.insert(i_l, insert_free);
            }
        } // else, nothing to insert! perfect slot :)

        //print_fs(linear_fs);
        first_empty = linear_fs.iter().enumerate().find(|(i,block)| block.typ == BlockType::Free);
        last_digit =  linear_fs.iter().enumerate().rev().find(|(i,block)| block.typ != BlockType::Free);
    };
}

fn solve_abs(linear_fs: &mut Vec<Block>) {
    // This time, attempt to move whole files to the leftmost span of free space blocks that could
    // fit the file. Attempt to move each file exactly once in order of decreasing file ID number
    // starting with the file with the highest file ID number. If there is no span of free space to
    // the left of a file that is large enough to fit the file, the file does not move.
    let linear_fs_tmp = linear_fs.clone(); // aaargh! janky!
    let mut sorted_files: Vec<&Block> = linear_fs_tmp.iter()
        .filter(|block| block.typ != BlockType::Free)
        .collect();

    sorted_files.sort_by(|lhs,rhs| {
        if let (BlockType::File(id_lhs), BlockType::File(id_rhs)) = (lhs.typ,rhs.typ) {
            id_rhs.cmp(&id_lhs)
        } else {
            Ordering::Equal
        }
    });

    for block in sorted_files {
        // go through each file block and try to put it in the FIRST acceptable free block thats
        // closest to 0.
        let i_l = linear_fs.iter().position(|b| b.eq(&block)).unwrap();

        let first_empty = linear_fs.iter().enumerate()
            .find(|(_,first_block)| first_block.typ == BlockType::Free
                && first_block.quantity >= block.quantity
            );

        if let Some((i_f, _)) = first_empty {
            if i_f >= i_l { continue; }
            let mut new_bucket = linear_fs[i_l].clone();
            let result = linear_fs[i_f].fill_from(&mut new_bucket);
            linear_fs[i_l] = new_bucket;

            if let (Some(free_block), true) = result {
                linear_fs.insert(i_f+1, free_block);
            }
        }
        //print_fs(linear_fs);
    }
}

fn checksum(linear_fs: &Vec<Block>) -> u64 {
    let mut block_pos: u32 = 0;
    let mut sum: u64 = 0;
    linear_fs.iter()
        .map(|block| {
            block_pos += block.quantity;
            if let BlockType::File(id) = block.typ {
                let r =((block_pos-block.quantity)..block_pos)
                    .fold(0,|l,r| l+r) as u64 * id as u64;
                r
            } else {
                0
            }
    }).sum()
}

fn print_fs(linear_fs: &Vec<Block>) {
    linear_fs.iter().for_each(|c| print!("{c}"));
    println!("");
}