use core::fmt;
use std::{fmt::Write, fs::File, io::{stdout, Empty, Read}, thread::{sleep, sleep_ms}, time::Duration};

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
enum Direction {
    Up,Down,Left,Right
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn move_forward(&mut self, grid: &mut Vec<Vec<Node>>) -> Option<FailState> {
        if let (Some(n_x),Some(n_y)) = self.pos_ahead(grid) {
            let node = grid[n_y][n_x];
            if node == Node::Obstacle { return Some(FailState::Obstacle); }

            if let Node::Visited(n) = node { // update visitor count on entry
                grid[n_y][n_x] = Node::Visited(n+1);
            } else {
                grid[n_y][n_x] = Node::Visited(1);
            }

            self.x = n_x;
            self.y = n_y;
            None
        } else {
            Some(FailState::OOB)
        }
    }
    fn turn_clockwise_90(&mut self) -> Direction {
        self.direction = match &self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        self.direction
    }
    fn pos_ahead(&self, grid: &Vec<Vec<Node>>) -> (Option<usize>, Option<usize>) {
        let mut x: Option<usize> = Some(self.x);
        let mut y: Option<usize> = Some(self.y);
        match self.direction {
            Direction::Up => y = self.y.checked_sub(1),
            Direction::Down => y = self.y.checked_add(1),
            Direction::Left => x = self.x.checked_sub(1),
            Direction::Right => x = self.x.checked_add(1),
        };
        if let Some(_y) = y {
            if _y >= grid.len() { y = None; }
        } 
        if let Some(_x) = x {
            if _x >= grid[0].len() { x = None; }
        } 
        (x,y)
    }
    fn here(&self, grid: &Vec<Vec<Node>>) -> Node {
        grid[self.y][self.x]
    }
}

// this is for pretty grid
impl fmt::Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(
            match &self.direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>'
            }
        )?;
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Obstacle,
    Visited(u32),
    Empty,
}

// this is for pretty grid
impl fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(
            match &self {
                Node::Obstacle => '#',
                Node::Visited(_) => 'X',
                Node::Empty => '.',
            }
        )
    }
}

#[derive(PartialEq, Eq)]
enum FailState {
    Obstacle,
    OOB,
}

pub fn run(animate:bool) {
    let mut file = File::open("./input/day6").expect("no read :(");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("weeeeh");
    let mut base_guard: Guard = Guard { x:0,y:0,direction:Direction::Up};
    let base_maze: Vec<Vec<Node>> = string.lines().enumerate()
        .map(|(i, string)| string.chars().enumerate()
            .map(|(j,c)| {
                let mut _guard: Option<Guard> = None;
                match c {
                    '#' => Node::Obstacle,
                    '.' => Node::Empty,
                    '^' => {
                        base_guard = Guard {
                            x: j,
                            y: i,
                            direction: Direction::Up
                        };
                        Node::Visited(1)
                    },
                    'v' => {
                        base_guard = Guard {
                            x: j,
                            y: i,
                            direction: Direction::Down
                        };
                        Node::Visited(1)
                    },
                    '<' => {
                        base_guard = Guard {
                            x: j,
                            y: i,
                            direction: Direction::Left
                        };
                        Node::Visited(1)
                    },
                    '>' => {
                        base_guard = Guard {
                            x: j,
                            y: i,
                            direction: Direction::Right
                        };
                        Node::Visited(1)
                    },
                    _ => Node::Empty
                }
            }).collect()
        ).collect();

    let mut guard_part1 = base_guard.clone();
    let mut maze_part1 = base_maze.clone();

    println!("{}\n", play_grid_to_string(&base_maze, &base_guard));
    let mut failed = false;
    while !failed { // keep going until guard failed (oob)
        let state = guard_part1.move_forward(&mut maze_part1);
        if let Some(fail) = guard_part1.move_forward(&mut maze_part1) {
            if fail == FailState::OOB { failed = true; break; }
            guard_part1.turn_clockwise_90();
        }
        if (animate) {
            clear();
            println!("{}", play_grid_to_string(&maze_part1, &guard_part1));
        }
    }

    //println!("{}", play_grid_to_string(&maze_part1, &guard_part1));

    let n_visited: u32 = maze_part1.iter().map(|line| line.iter().filter(|n| if let Node::Visited(_) = n { true } else { false }).collect::<Vec<_>>().len() as u32).sum();
    println!("PART 1: {n_visited}");

    if animate { pause(); }

    // THIS SOLUTION IS MADLY INEFFICIENT BUT IT SHOULD WORK
    let mut n_solutions = 0;
    let mut guards_part2 = vec![vec![base_guard.clone();base_maze[0].len()]; base_maze.len()];
    let mut maze_part2 = vec![vec![base_maze.clone();base_maze[0].len()]; base_maze.len()];
    maze_part2.iter_mut().enumerate().skip(8).for_each(|(i,row)| {
        row.iter_mut().enumerate().for_each(|(j, maze)| {
            maze[i][j] = Node::Obstacle;

            let mut failed = false;
            let mut cursor = guards_part2[i][j];
            while !failed { // keep going until guard failed (oob)
                let state = cursor.move_forward(maze);
                if let Some(fail) = cursor.move_forward(maze) {
                    if fail == FailState::OOB { failed = true; break; }
                    if let Node::Visited(n) = cursor.here(maze) {
                        if (n > 3) { // if >3 visits then it must be trapped traversing this ndoe in at least one direction, if ti traverses it in another direction
                            failed = false; break;
                        }
                    }
                    cursor.turn_clockwise_90();
                }
            }
            if (animate) {
                clear();
                println!("{}", play_grid_to_string(maze, &cursor));
                //sleep(Duration::from_millis(2));
            }

            if (!failed) {
                //println!("{}", play_grid_to_string(maze, &cursor));
                n_solutions += 1;
            }
        });
    });
    println!("PART2: {n_solutions}");

}

fn grid_to_string(grid: &Vec<Vec<Node>>) -> String {
    grid.iter()
        .map(|line| {
            line.iter()
                .fold(String::new(), |string,node| string + &node.to_string())
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn play_grid_to_string(grid: &Vec<Vec<Node>>, cursor: &Guard) -> String {
    grid.iter().enumerate()
        .map(|(i,line)| {
            line.iter().enumerate()
                .fold(String::new(), |string,(j,node)| {
                    if cursor.x == j && cursor.y == i {
                        string + &cursor.to_string()
                    } else {
                        string + &node.to_string()
                    }
                })
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn pause() {
    println!("Press enter to continue");

    let mut _buf = String::new();

    std::io::stdin()
        .read_line(&mut _buf)
        .expect("Failed to read line");
}

fn clear() {
    print!("{esc}c", esc = 27 as char);
}