use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let input = read_lines_as_str("./day4.input")[0]
        .split("-")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let min = input[0];
    let max = input[1];
    let answer1 = solve1(min, max);
    let answer2 = solve2(min, max);
    println!("Day 4 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(min: u32, max: u32) -> u32 {
    let mut possible_pw = 0;
    for i in min..=max {
        if has_two_same_adj(i) && never_dec(i) {
            possible_pw += 1;
        }
    }
    possible_pw
}

fn has_two_same_adj(num: u32) -> bool {
    let digits = num.to_string().chars().collect::<Vec<char>>();
    digits
        .iter()
        .enumerate()
        .skip(1)
        .any(|(i, digit)| digits[i - 1] == *digit)
}

fn never_dec(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|a| a.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    digits
        .iter()
        .enumerate()
        .skip(1)
        .all(|(i, digit)| digits[i - 1] <= *digit)
}

fn solve2(min: u32, max: u32) -> u32 {
    let mut possible_pw = 0;
    for i in min..=max {
        if has_two_same_adj_mod(i) && never_dec(i) {
            possible_pw += 1;
        }
    }
    possible_pw
}

fn has_two_same_adj_mod(num: u32) -> bool {
    let digits = num.to_string().chars().collect::<Vec<char>>();
    digits
        .iter()
        .any(|digit| digits.iter().filter(|d| *d == digit).count() == 2)
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
