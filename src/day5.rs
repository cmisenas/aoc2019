use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day5.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let answer1 = solve1(program.clone());
    let answer2 = solve2(program.clone());
    println!("Day 5 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut program: Vec<i32>) -> i32 {
    let air_conditioner_id = 1;
    let mut index = 0;
    let mut diagnostics_code = 0;
    loop {
        let opcode = program[index] % 100;
        let mode1 = (program[index] / 100) % 10;
        let mode2 = program[index] / 1000;
        match opcode {
            1 | 2 => {
                let param1 = match mode1 {
                    1 => program[index + 1],
                    _ => program[program[index + 1] as usize],
                };
                let param2 = match mode2 {
                    1 => program[index + 2],
                    _ => program[program[index + 2] as usize],
                };
                let store = program[index + 3] as usize; // ALWAYS POSITION MODE
                let result = match opcode {
                    1 => param1 + param2,
                    _ => param1 * param2,
                };
                program[store] = result;
                index += 4;
            }
            3 | 4 => {
                // ALWAYS POSITION MODE
                let addr = program[index + 1] as usize;
                match opcode {
                    // Input the value
                    3 => program[addr] = air_conditioner_id,
                    // Output the value
                    _ => diagnostics_code = program[addr],
                };
                index += 2;
            }
            99 => break,
            _ => break,
        };
    }
    diagnostics_code
}

fn solve2(mut program: Vec<i32>) -> i32 {
    let thermal_radiator_id = 5;
    let mut index = 0;
    let mut diagnostics_code = 0;
    loop {
        let opcode = program[index] % 100;
        let mode1 = (program[index] / 100) % 10;
        let mode2 = program[index] / 1000;
        match opcode {
            1 | 2 => {
                let param1 = match mode1 {
                    1 => program[index + 1],
                    _ => program[program[index + 1] as usize],
                };
                let param2 = match mode2 {
                    1 => program[index + 2],
                    _ => program[program[index + 2] as usize],
                };
                let store = program[index + 3] as usize; // ALWAYS POSITION MODE
                let result = match opcode {
                    1 => param1 + param2,
                    _ => param1 * param2,
                };
                program[store] = result;
                index += 4;
            }
            3 | 4 => {
                // ALWAYS POSITION MODE
                let addr = program[index + 1] as usize;
                match opcode {
                    // Input the value
                    3 => program[addr] = thermal_radiator_id,
                    // Output the value
                    _ => diagnostics_code = program[addr],
                };
                index += 2;
            }
            5 | 6 => {
                let param1 = match mode1 {
                    1 => program[index + 1],
                    _ => program[program[index + 1] as usize],
                };
                let param2 = match mode2 {
                    1 => program[index + 2],
                    _ => program[program[index + 2] as usize],
                };
                let jump_if_true = opcode == 5 && param1 != 0;
                let jump_if_false = opcode == 6 && param1 == 0;
                if jump_if_true || jump_if_false {
                    index = param2 as usize;
                } else {
                    index += 3;
                }
            }
            7 | 8 => {
                let param1 = match mode1 {
                    1 => program[index + 1],
                    _ => program[program[index + 1] as usize],
                };
                let param2 = match mode2 {
                    1 => program[index + 2],
                    _ => program[program[index + 2] as usize],
                };
                let store = program[index + 3] as usize; // ALWAYS POSITION MODE
                let less_than = opcode == 7 && param1 < param2;
                let equal = opcode == 8 && param1 == param2;
                if less_than || equal {
                    program[store] = 1;
                } else {
                    program[store] = 0;
                }
                index += 4;
            }
            99 => break,
            _ => break,
        };
    }
    diagnostics_code
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

fn read_lines_as_int<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<i32>().unwrap())
        .collect()
}
