use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let lines = read_lines_as_str("./day3.input");
    let wires = lines
        .iter()
        .map(|l| l.split(",").map(|p| p.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let wire_paths = parse_wires(&wires);
    let intersections = get_wire_intersections(&wire_paths);
    let answer1 = solve1(&intersections);
    let answer2 = solve2(&intersections, &wire_paths);
    println!("Day 3 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn get_wire_intersections(wire_paths: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut intersections: Vec<(i32, i32)> = Vec::new();
    for (i, wire) in wire_paths.iter().enumerate() {
        for other_wire in wire_paths.iter().skip(i + 1) {
            intersections = [intersections, get_intersections(&wire, &other_wire)].concat();
        }
    }
    intersections
}

fn parse_wires(wires: &Vec<Vec<String>>) -> Vec<Vec<(i32, i32)>> {
    let mut wire_paths: Vec<Vec<(i32, i32)>> = Vec::new();
    for wire in wires {
        // Start at origin
        let mut x = 0;
        let mut y = 0;
        let mut curr_wire: Vec<(i32, i32)> = Vec::new();
        curr_wire.push((x, y));
        for path in wire {
            let (new_x, new_y) = parse_dir(path.to_string());
            x += new_x;
            y += new_y;
            curr_wire.push((x, y));
        }
        wire_paths.push(curr_wire);
    }
    wire_paths
}

fn solve1(intersections: &Vec<(i32, i32)>) -> u32 {
    let min = intersections
        .iter()
        .filter(|i| !(i.0 == 0 && i.1 == 0))
        .min_by(|x, y| (x.0.abs() + x.1.abs()).cmp(&(y.0.abs() + y.1.abs())))
        .unwrap();
    (min.0.abs() + min.1.abs()) as u32
}

fn get_intersections(wire_a: &Vec<(i32, i32)>, wire_b: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut intersections: Vec<(i32, i32)> = Vec::new();
    for (i, path_a) in wire_a.iter().enumerate().skip(1) {
        for (j, path_b) in wire_b.iter().enumerate().skip(1) {
            let start_a = wire_a[i - 1];
            let start_b = wire_b[j - 1];
            if let Some(intersection) = check_intersects((start_a, *path_a), (start_b, *path_b)) {
                intersections.push(intersection);
            }
        }
    }
    intersections
}

fn check_intersects(
    wire_a: ((i32, i32), (i32, i32)),
    wire_b: ((i32, i32), (i32, i32)),
) -> Option<(i32, i32)> {
    // wire_b is horizontal and wire_a is vertical
    if min(wire_b.0 .0, wire_b.1 .0) <= wire_a.0 .0
        && max(wire_b.0 .0, wire_b.1 .0) >= wire_a.0 .0
        && min(wire_a.0 .1, wire_a.1 .1) <= wire_b.0 .1
        && max(wire_a.0 .1, wire_a.1 .1) >= wire_b.0 .1
    {
        Some((wire_a.0 .0, wire_b.0 .1))
    } else if min(wire_a.0 .0, wire_a.1 .0) <= wire_b.0 .0
        && max(wire_a.0 .0, wire_a.1 .0) >= wire_b.0 .0
        && min(wire_b.0 .1, wire_b.1 .1) <= wire_a.0 .1
        && max(wire_b.0 .1, wire_b.1 .1) >= wire_a.0 .1
    {
        Some((wire_b.0 .0, wire_a.0 .1))
    } else {
        None
    }
}

fn parse_dir(mut dir: String) -> (i32, i32) {
    let steps = dir.split_off(1).parse::<i32>().unwrap();
    match dir.as_str() {
        "D" => (0, -steps),
        "R" => (steps, 0),
        "U" => (0, steps),
        "L" => (-steps, 0),
        _ => (0, 0),
    }
}

fn solve2(intersections: &Vec<(i32, i32)>, wire_paths: &Vec<Vec<(i32, i32)>>) -> u32 {
    intersections
        .iter()
        .filter(|i| !(i.0 == 0 && i.1 == 0))
        .map(|i| get_steps(*i, &wire_paths[0]) + get_steps(*i, &wire_paths[1]))
        .min()
        .unwrap()
}

fn get_steps(point: (i32, i32), wire: &Vec<(i32, i32)>) -> u32 {
    let mut steps = 0;
    for (i, end_path) in wire.iter().enumerate().skip(1) {
        let start_path = wire[i - 1];
        if is_between_line(point, start_path, *end_path) {
            steps += (start_path.0 - point.0).abs();
            steps += (start_path.1 - point.1).abs();
            break;
        } else {
            steps += (start_path.0 - end_path.0).abs();
            steps += (start_path.1 - end_path.1).abs();
        }
    }
    steps as u32
}

fn is_between_line(point: (i32, i32), start: (i32, i32), end: (i32, i32)) -> bool {
    (point.0 == end.0
        && point.0 == start.0
        && point.1 <= max(end.1, start.1)
        && point.1 >= min(end.1, start.1))
        || (point.1 == end.1
            && point.1 == start.1
            && point.0 <= max(end.0, start.0)
            && point.0 >= min(end.0, start.0))
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
