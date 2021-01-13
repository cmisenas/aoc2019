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
        .collect::<Vec<Vec<i16>>>();
    let answer1 = solve1(moons.clone());
    let answer2 = solve2(moons.clone());
    println!("Day 12 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut moons: Vec<Vec<i16>>) -> u16 {
    let mut moon_velocity = vec![vec![0; 3]; moons.len()];
    for _ in 0..1000 {
        for (a, moon_a) in moons.iter().enumerate() {
            for (b, moon_b) in moons.iter().enumerate().skip(a + 1) {
                for (i, (dim_a, dim_b)) in moon_a.iter().zip(moon_b.iter()).enumerate() {
                    if dim_a > dim_b {
                        moon_velocity[b][i] += 1;
                        moon_velocity[a][i] -= 1;
                    } else if dim_a < dim_b {
                        moon_velocity[a][i] += 1;
                        moon_velocity[b][i] -= 1;
                    }
                }
            }
        }
        for (i, moon) in moons.iter_mut().enumerate() {
            moon[0] += moon_velocity[i][0];
            moon[1] += moon_velocity[i][1];
            moon[2] += moon_velocity[i][2];
        }
    }
    calculate_total_energy(&moons, &moon_velocity)
}

fn calculate_total_energy(moons: &Vec<Vec<i16>>, velocities: &Vec<Vec<i16>>) -> u16 {
    moons.iter().zip(velocities.iter()).fold(0, |acc, (m, v)| {
        acc + (m.iter().map(|a| a.abs() as u16).sum::<u16>()
            * v.iter().map(|a| a.abs() as u16).sum::<u16>())
    })
}

fn solve2(moons: Vec<Vec<i16>>) -> usize {
    use std::cmp::max;

    let x = subset_cycle(0, moons.clone());
    let y = subset_cycle(1, moons.clone());
    let z = subset_cycle(2, moons.clone());
    let inc = max(max(x, y), z);
    let mut loops: usize = 0;
    loop {
        loops += inc;
        if loops % x == 0 && loops % y == 0 && loops % z == 0 {
            break;
        }
    }
    loops
}

fn subset_cycle(dimension: usize, mut moons: Vec<Vec<i16>>) -> usize {
    let mut moon_velocity = vec![0; moons.len()];
    let mut loops = 0;
    let mut initial_config = String::new();
    for moon in moons.iter() {
        initial_config.push_str(&moon[dimension].to_string());
        initial_config.push_str("0");
    }
    loop {
        loops += 1;
        let mut str_config = String::new();
        for (a, moon_a) in moons.iter().enumerate() {
            for (b, moon_b) in moons.iter().enumerate().skip(a + 1) {
                if moon_a[dimension] > moon_b[dimension] {
                    moon_velocity[b] += 1;
                    moon_velocity[a] -= 1;
                } else if moon_a[dimension] < moon_b[dimension] {
                    moon_velocity[a] += 1;
                    moon_velocity[b] -= 1;
                }
            }
        }
        for (i, moon) in moons.iter_mut().enumerate() {
            moon[dimension] += moon_velocity[i];
            str_config.push_str(&moon[dimension].to_string());
            str_config.push_str(&moon_velocity[i].to_string());
        }
        if initial_config == str_config {
            break;
        }
    }
    loops
}

fn parse_moon(moon: String) -> Vec<i16> {
    moon.strip_prefix("<")
        .unwrap()
        .strip_suffix(">")
        .unwrap()
        .split(", ")
        .map(|a| a.split("=").nth(1).unwrap().parse::<i16>().unwrap())
        .collect::<Vec<i16>>()
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
