use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_int("./day1.input");
    let answer1 = solve1(&lines);
    let answer2 = solve2(&lines);
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(lines: &[f32]) -> u32 {
    lines
        .iter()
        .map(|module| (module / 3f32).floor() as u32 - 2)
        .sum()
}

fn solve2(lines: &[f32]) -> u32 {
    lines
        .iter()
        .map(|module| {
            let mut fuel_req: u32 = 0;
            let mut module_fuel = *module as i32;
            loop {
                module_fuel = (module_fuel as f32 / 3f32).floor() as i32 - 2;
                if module_fuel <= 0 {
                    break;
                }
                fuel_req += module_fuel as u32;
            }
            fuel_req
        })
        .sum()
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

fn read_lines_as_int<P>(filename: P) -> Vec<f32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<f32>().unwrap())
        .collect()
}
