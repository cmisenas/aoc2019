use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day6.input");
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for line in &lines {
        let o = line.split(")").collect::<Vec<&str>>();
        orbits
            .entry(o[0].to_string())
            .or_insert_with(Vec::new)
            .push(o[1].to_string());
    }
    let answer1 = solve1(&orbits);
    let answer2 = solve2(&lines);
    println!("Day 6 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(orbits: &HashMap<String, Vec<String>>) -> u32 {
    count_orbits(0, String::from("COM"), orbits)
}

fn count_orbits(sum: u32, orbit: String, orbits: &HashMap<String, Vec<String>>) -> u32 {
    if orbits.contains_key(&orbit) {
        let next_orbits = orbits.get(&orbit).unwrap();
        let mut o_sum = 0;
        for n_orbit in next_orbits {
            o_sum += count_orbits(sum + 1, n_orbit.to_string(), &orbits);
        }
        o_sum + sum
    } else {
        sum
    }
}

fn solve2(lines: &[String]) -> u32 {
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
