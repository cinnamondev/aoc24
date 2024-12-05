use std::{fs::File, io::Read};

enum DIRECTION {
    HORIZONTAL,
    DIAGONAL,
    VERTICAL
}
// 5 am stuff... very brute 
pub fn run() {
    let mut file = File::open("./input/day4").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");

    let mut n_chars_line = 0; 
    let matrix: Vec<Vec<char>> = string.split_whitespace()
        .map(|s| {
            let chars: Vec<char> = s.chars().collect::<>();
            n_chars_line = chars.len(); // this is kind of janky and done too many times but it was a really convenient point to pull it from.
            return chars;
        } )
        .collect::<>();


    let mut location_list: Vec<(usize,usize,DIRECTION,bool)> = Vec::new();
    let n_lines = matrix.len();

    let mut grid = vec![vec!['.'; n_chars_line]; n_lines];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut word = String::with_capacity(4);
            // traverse node horizontal (right)
            if (j+3 < matrix[i].len()) {
                word = (matrix[i][j..=j+3]).iter().collect();
                if (word == "XMAS" || word == "SAMX") {
                    location_list.push((i,j,DIRECTION::HORIZONTAL, word == "XMAS"));
                    let mut iterator = word.chars();
                    for k in j..j+4 {
                        grid[i][k] = iterator.next().unwrap();
                    }                   
                }
            }
            // traverse node vertical (down)
            if (i+3 < matrix.len()) {
                word = String::with_capacity(4);
                for k in i..=i+3 {
                    word.push(matrix[k][j]);
                }
                if (word == "XMAS" || word == "SAMX") {
                    location_list.push((i,j,DIRECTION::VERTICAL, word == "XMAS"));
                    let mut iterator = word.chars();
                    for k in i..i+4 {
                        grid[k][j] = iterator.next().unwrap();
                    }                   
                }
            }

            // traverse diagonal (d+r)
            if ((i+3 < matrix.len()) && (j+3 < matrix[i].len())) {
                word = String::with_capacity(4);
                for k in 0..=3 {
                    word.push(matrix[i+k][j+k]);
                }
                if (word == "XMAS" || word == "SAMX") {
                    location_list.push((i,j,DIRECTION::DIAGONAL, word == "XMAS"));
                    let mut iterator = word.chars();
                    for k in 0..4 {
                        grid[i+k][j+k] = iterator.next().unwrap();
                    }                   
                }
            }

            // diagonal (d+l)
            if ((i+3 < matrix.len()) && (j.checked_sub(3).is_some())) {
                word = String::new();
                for k in 0..=3 {
                    word.push(matrix[i+k][j-k]);
                }
                if (word == "XMAS" || word == "SAMX") {
                    location_list.push((i,j,DIRECTION::DIAGONAL, word == "XMAS"));
                    let mut iterator = word.chars();
                    for k in 0..=3 {
                        grid[i+k][j-k] = iterator.next().unwrap();
                    }                   
                }
            }
        }
    }
    // explore each node in each direction seperately... (horizontal scan / line, vertical scan / row, diagonal scan /c )

    let out_string = grid.iter()
        .map(|chars| chars.into_iter().collect::<String>())
        .reduce(|current_line, next_line| format!("{}\n{}",current_line,next_line))
        .unwrap();

    println!("{}",out_string);

    println!("{}", location_list.len()); // im annoyed that part 2 doesnt need this now because i made it all nice

    // part 2
    let mut counter = 0;
    let mut grid_pt2 = vec![vec!['.'; n_chars_line]; n_lines];
    for i in 1..matrix.len()-1 {
        for j in 1..matrix[i].len()-1 {
            let diag_DR = false;
            let diag_DL = false;
            if (matrix[i][j] == 'A') {
                let mut right = String::new();
                right.push(matrix[i+1][j-1]);
                right.push(matrix[i-1][j+1]);

                let mut left: String = String::new();
                left.push(matrix[i-1][j-1]);
                left.push(matrix[i+1][j+1]);

                if ((left == "MS" || left == "SM") && (right == "MS" || right == "SM")) {
                    counter+=1;
                }
            }
        }
    }
    println!("{}",counter);

}
