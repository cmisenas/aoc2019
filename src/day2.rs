use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day2.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let answer1 = solve1(&program);
    let answer2 = solve2(&program);
    println!("Day 2 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(program: &Vec<u32>) -> u32 {
    run_program(12, 2, program.clone())
}

fn solve2(program: &Vec<u32>) -> u32 {
    let mut result = 0;
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(noun, verb, program.clone()) == 19690720 {
                result = 100 * noun + verb;
            }
        }
    }
    result
}

fn run_program(noun: u32, verb: u32, mut program: Vec<u32>) -> u32 {
    // Replay according to puzzle desc
    program[1] = noun;
    program[2] = verb;
    let mut index = 0;
    loop {
        let param1 = program[index + 1] as usize;
        let param2 = program[index + 2] as usize;
        let store = program[index + 3] as usize;
        match program[index] {
            1 => {
                let result = program[param1] + program[param2];
                program[store] = result;
            }
            2 => {
                let result = program[param1] * program[param2];
                program[store] = result;
            }
            99 => break,
            _ => break,
        };
        index += 4;
    }
    program[0]
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

fn read_lines_as_int<P>(filename: P) -> Vec<u32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<u32>().unwrap())
        .collect()
}
