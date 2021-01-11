use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day10.input");
    let space_map = lines
        .iter()
        .map(|line| {
            line.split("")
                .filter_map(|l| match l == "" {
                    true => None,
                    false => Some(l.to_string()),
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    let mut asteroids: Vec<(usize, usize)> = Vec::new();
    for (y, space) in space_map.iter().enumerate() {
        for (x, space_row) in space.iter().enumerate() {
            if space_row == "#" {
                asteroids.push((x, y))
            }
        }
    }
    let answer1 = solve1(&asteroids);
    let answer2 = solve2(&lines);
    println!("Day 10 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(asteroids: &Vec<(usize, usize)>) -> usize {
    let scores = asteroids
        .iter()
        .map(|asteroid| {
            (
                asteroid,
                calc_asteroid_score(asteroid.0, asteroid.1, asteroids),
            )
        })
        .collect::<Vec<(&(usize, usize), usize)>>();

    let max_score = scores.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("{:?}", max_score);
    max_score.1
}

fn calc_asteroid_score(x: usize, y: usize, asteroids: &Vec<(usize, usize)>) -> usize {
    let mut total_in_los: HashMap<String, (usize, usize)> = HashMap::new();
    for asteroid in asteroids.iter() {
        if asteroid.0 == x && asteroid.1 == y {
            continue;
        }
        let x_distance = (x + 1) as i16 - (asteroid.0 + 1) as i16;
        let y_distance = (y + 1) as i16 - (asteroid.1 + 1) as i16;
        let ratio_distance = simplify_fraction(y_distance, x_distance);
        if let Some(asteroid_in_los) = total_in_los.get(&ratio_distance) {
            // Replace with whatever is nearer to it
            let curr_asteroid_distance =
                (asteroid.0 as i16 - x as i16).abs() + (asteroid.1 as i16 - y as i16).abs();
            let asteroid_in_los_distance = (asteroid_in_los.0 as i16 - x as i16).abs()
                + (asteroid_in_los.1 as i16 - y as i16).abs();
            if curr_asteroid_distance < asteroid_in_los_distance {
                total_in_los.insert(ratio_distance, *asteroid);
            }
        } else {
            total_in_los.insert(ratio_distance, *asteroid);
        }
    }
    total_in_los.len()
}

fn simplify_fraction(mut numerator: i16, mut denominator: i16) -> String {
    let mut frac_gcd = gcd(numerator.abs() as u16, denominator.abs() as u16) as i16;
    let mut fraction = String::from("");
    fraction.push_str(&(numerator / frac_gcd).to_string());
    fraction.push_str("/");
    fraction.push_str(&(denominator / frac_gcd).to_string());
    fraction
}

fn gcd(mut num: u16, mut den: u16) -> u16 {
    if num == 0 {
        return den;
    } else if den == 0 {
        return num;
    } else if den == num {
        return den;
    }

    if num % 2 == 0 {
        if den % 2 == 1 {
            return gcd(num / 2, den);
        } else {
            return 2 * gcd(num / 2, den / 2);
        }
    } else {
        if den % 2 == 0 {
            return gcd(num, den / 2);
        }

        if num > den {
            return gcd((num - den) / 2, den);
        } else {
            return gcd((den - num) / 2, num);
        }
    }
}

fn solve2(lines: &[String]) -> u8 {
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
