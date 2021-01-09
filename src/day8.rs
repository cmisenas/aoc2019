use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let input = read_lines_as_str("./day8.input");
    let digits = input[0]
        .split("")
        .filter_map(|d| match d != "" {
            true => Some(d.parse::<u8>().unwrap()),
            false => None,
        })
        .collect::<Vec<u8>>();
    let mut layers: Vec<Vec<Vec<u8>>> = Vec::new();
    let width = 25;
    let height = 6;
    let mut index = 0;
    while index < digits.len() {
        let mut layer = Vec::new();
        for i in (0..height) {
            let mut row = Vec::new();
            for j in (0..width) {
                row.push(digits[index]);
                index += 1;
            }
            layer.push(row);
        }
        layers.push(layer);
    }
    let answer1 = solve1(&layers);
    let answer2 = solve2(height, width, &layers);
    println!("Day 8 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(layers: &Vec<Vec<Vec<u8>>>) -> usize {
    let found_layer = layers
        .iter()
        .min_by(|layer1, layer2| {
            layer1
                .iter()
                .flatten()
                .filter(|l| **l == 0)
                .count()
                .cmp(&layer2.iter().flatten().filter(|l| **l == 0).count())
        })
        .unwrap()
        .iter()
        .flatten()
        .collect::<Vec<&u8>>();
    let ones = found_layer.iter().filter(|l| ***l == 1).count();
    let twos = found_layer.iter().filter(|l| ***l == 2).count();
    ones * twos
}

fn solve2(height: usize, width: usize, layers: &Vec<Vec<Vec<u8>>>) -> String {
    let mut final_msg: Vec<Vec<u8>> = Vec::new();
    for i in 0..height {
        let mut layer: Vec<u8> = Vec::new();
        for j in 0..width {
            let mut layer_i = 0;
            while layers[layer_i][i][j] == 2 {
                layer_i += 1;
            }
            layer.push(layers[layer_i][i][j]);
        }
        final_msg.push(layer);
    }
    let mut final_msg_str = final_msg
        .iter()
        .map(|l| {
            l.iter()
                .map(|a| match *a == 1 {
                    true => "█",
                    false => " ",
                })
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");
    final_msg_str.insert_str(0, "\n");
    final_msg_str
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
