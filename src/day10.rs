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
    let best_asteroid = get_best_asteroid(&asteroids);
    let answer1 = best_asteroid.1.len();
    let answer2 = solve2(*best_asteroid.0, best_asteroid.1, &asteroids);
    println!("Day 10 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn get_best_asteroid(
    asteroids: &Vec<(usize, usize)>,
) -> (&(usize, usize), HashMap<String, (usize, usize)>) {
    let scores = asteroids
        .iter()
        .map(|asteroid| {
            (
                asteroid,
                get_asteroids_los(asteroid.0, asteroid.1, asteroids),
            )
        })
        .collect::<Vec<(&(usize, usize), HashMap<String, (usize, usize)>)>>();

    let best_asteroid = scores
        .iter()
        .max_by(|a, b| a.1.len().cmp(&b.1.len()))
        .unwrap();
    best_asteroid.clone()
}

fn get_asteroids_los(
    x: usize,
    y: usize,
    asteroids: &Vec<(usize, usize)>,
) -> HashMap<String, (usize, usize)> {
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
    total_in_los
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

fn solve2(
    best_asteroid: (usize, usize),
    asteroid_los: HashMap<String, (usize, usize)>,
    asteroids: &Vec<(usize, usize)>,
) -> usize {
    use std::cmp::Ordering;
    use std::f64::{INFINITY, NEG_INFINITY};
    let mut sorted_asteroids = asteroid_los
        .iter()
        .map(|(distance, asteroid)| (distance.to_string(), asteroid.clone()))
        .collect::<Vec<(String, (usize, usize))>>();

    sorted_asteroids.sort_by(|(distance_a, _), (distance_b, _)| {
        let distance_a_ = distance_a
            .split("/")
            .map(|a| a.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        let distance_b_ = distance_b
            .split("/")
            .map(|a| a.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        let a_quadrant = get_quadrant(distance_a_[1], distance_a_[0]);
        let b_quadrant = get_quadrant(distance_b_[1], distance_b_[0]);
        if a_quadrant == b_quadrant {
            let calc_distance_a = distance_a_[0] as f64 / distance_a_[1] as f64;
            let calc_distance_b = distance_b_[0] as f64 / distance_b_[1] as f64;
            calc_distance_a.partial_cmp(&calc_distance_b).unwrap()
        } else {
            a_quadrant.cmp(&b_quadrant)
        }
    });
    let asteroid_200 = sorted_asteroids[199].1;
    asteroid_200.0 * 100 + asteroid_200.1
}

fn get_quadrant(x: i8, y: i8) -> u8 {
    // N = 0
    // NE = 1
    // E = 2
    // SE = 3
    // S = 4
    // SW = 5
    // W = 6
    // NW = 7
    if x == 0 && y > 0 {
        0
    } else if x < 0 && y > 0 {
        1
    } else if y == 0 && x < 0 {
        2
    } else if x < 0 && y < 0 {
        3
    } else if x == 0 && y < 0 {
        4
    } else if x > 0 && y < 0 {
        5
    } else if y == 0 && x > 0 {
        6
    } else if y > 0 && x > 0 {
        7
    } else {
        panic!("Quadrant not found");
    }
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
