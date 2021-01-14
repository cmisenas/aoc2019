use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::intcode_computer::IntcodeComputer;

pub fn main() {
    let lines = read_lines_as_str("./day13.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let answer1 = solve1(program.clone());
    let answer2 = solve2(program.clone());
    println!("Day 13 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut program: Vec<i64>) -> usize {
    let mut computer = IntcodeComputer::new(0, program.clone(), 0);
    let mut objects: HashMap<(i64, i64), i64> = HashMap::new();
    while !computer.has_halted {
        let x = computer.run_program();
        let y = computer.run_program();
        let obj = computer.run_program();
        objects.insert((x, y), obj);
    }
    objects.iter().filter(|(_, obj)| **obj == 2).count()
}

fn solve2(mut program: Vec<i64>) -> i64 {
    program[0] = 2;
    let mut computer = IntcodeComputer::new(0, program.clone(), 0);
    let mut score = 0;
    let mut paddle_pos = 0;
    let mut ball_pos = 0;
    while !computer.has_halted {
        let x = computer.run_program();
        let y = computer.run_program();
        let val = computer.run_program();
        if x == -1 && y == 0 {
            score = val;
        } else {
            if val == 3 {
                paddle_pos = x;
            } else if val == 4 {
                ball_pos = x;
            }
            if paddle_pos < ball_pos {
                computer.input = 1;
            } else if paddle_pos > ball_pos {
                computer.input = -1;
            } else {
                computer.input = 0;
            }
        }
    }
    score
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
