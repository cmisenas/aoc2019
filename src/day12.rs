extern crate regex;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day12.input");
    let moons = lines
        .iter()
        .map(|line| parse_moon(line.to_string()))
        .collect::<Vec<(i16, i16, i16)>>();
    let answer1 = solve1(moons.clone());
    let answer2 = solve2(&moons);
    println!("Day 12 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut moons: Vec<(i16, i16, i16)>) -> u16 {
    let mut moon_velocity = vec![(0, 0, 0); moons.len()];
    for _ in 0..1000 {
        for (a, moon_a) in moons.iter().enumerate() {
            for (b, moon_b) in moons.iter().enumerate().skip(a + 1) {
                if moon_a.0 > moon_b.0 {
                    moon_velocity[b].0 += 1;
                    moon_velocity[a].0 -= 1;
                } else if moon_a.0 < moon_b.0 {
                    moon_velocity[a].0 += 1;
                    moon_velocity[b].0 -= 1;
                }
                if moon_a.1 > moon_b.1 {
                    moon_velocity[b].1 += 1;
                    moon_velocity[a].1 -= 1;
                } else if moon_a.1 < moon_b.1 {
                    moon_velocity[a].1 += 1;
                    moon_velocity[b].1 -= 1;
                }
                if moon_a.2 > moon_b.2 {
                    moon_velocity[b].2 += 1;
                    moon_velocity[a].2 -= 1;
                } else if moon_a.2 < moon_b.2 {
                    moon_velocity[a].2 += 1;
                    moon_velocity[b].2 -= 1;
                }
            }
        }
        for (i, moon) in moons.iter_mut().enumerate() {
            moon.0 += moon_velocity[i].0;
            moon.1 += moon_velocity[i].1;
            moon.2 += moon_velocity[i].2;
        }
    }
    calculate_total_energy(&moons, &moon_velocity)
}

fn calculate_total_energy(moons: &Vec<(i16, i16, i16)>, velocities: &Vec<(i16, i16, i16)>) -> u16 {
    moons.iter().zip(velocities.iter()).fold(0, |acc, (m, v)| {
        acc + (m.0.abs() + m.1.abs() + m.2.abs()) as u16
            * (v.0.abs() + v.1.abs() + v.2.abs()) as u16
    })
}

fn solve2(moons: &Vec<(i16, i16, i16)>) -> u32 {
    0
}

fn parse_moon(moon: String) -> (i16, i16, i16) {
    let pos = moon
        .strip_prefix("<")
        .unwrap()
        .strip_suffix(">")
        .unwrap()
        .split(", ")
        .map(|a| a.split("=").nth(1).unwrap().parse::<i16>().unwrap())
        .collect::<Vec<i16>>();
    (pos[0], pos[1], pos[2])
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
