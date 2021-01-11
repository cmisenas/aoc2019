use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::intcode_computer::IntcodeComputer;

pub fn main() {
    let lines = read_lines_as_str("./day9.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let answer1 = solve1(program.clone());
    let answer2 = solve2(program.clone());
    println!("Day 9 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut program: Vec<i64>) -> i64 {
    let mut computer = IntcodeComputer::new(0, program.clone(), 1);
    computer.run_program()
}

fn solve2(mut program: Vec<i64>) -> i64 {
    let mut computer = IntcodeComputer::new(0, program.clone(), 2);
    computer.run_program()
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
