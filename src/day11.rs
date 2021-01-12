use crate::intcode_computer::IntcodeComputer;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    North,
    East,
    South,
    West,
}

pub fn main() {
    let lines = read_lines_as_str("./day11.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let answer1 = solve1(program.clone());
    let answer2 = solve2(program.clone());
    println!("Day 11 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut program: Vec<i64>) -> usize {
    let mut painted_panels: HashMap<(i16, i16), i8> = HashMap::new();
    let mut robot_coordinate = (0, 0);
    let mut robot_direction = Direction::North;
    let mut robot_computer = IntcodeComputer::new(0, program.clone(), 0);
    while !robot_computer.has_halted {
        if let Some(panel_paint) = painted_panels.get(&robot_coordinate) {
            if *panel_paint == 0i8 {
                robot_computer.input = 0;
            } else {
                robot_computer.input = 1;
            }
        } else {
            robot_computer.input = 0;
        }
        let paint = robot_computer.run_program();
        let turn = robot_computer.run_program();
        painted_panels.insert(robot_coordinate.clone(), paint as i8);

        robot_direction = turn_dir(robot_direction, turn == 1);
        robot_coordinate = match robot_direction {
            Direction::North => (robot_coordinate.0, robot_coordinate.1 - 1),
            Direction::East => (robot_coordinate.0 + 1, robot_coordinate.1),
            Direction::South => (robot_coordinate.0, robot_coordinate.1 + 1),
            Direction::West => (robot_coordinate.0 - 1, robot_coordinate.1),
        }
    }
    painted_panels.len()
}

fn turn_dir(start_dir: Direction, clockwise: bool) -> Direction {
    match start_dir {
        Direction::North => match clockwise {
            true => Direction::East,
            false => Direction::West,
        },
        Direction::East => match clockwise {
            true => Direction::South,
            false => Direction::North,
        },
        Direction::South => match clockwise {
            true => Direction::West,
            false => Direction::East,
        },
        Direction::West => match clockwise {
            true => Direction::North,
            false => Direction::South,
        },
    }
}

fn solve2(mut program: Vec<i64>) -> i64 {
    0
}

fn read_lines_as_str<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
