use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::intcode_computer::IntcodeComputer;

pub fn main() {
    let lines = read_lines_as_str("./day13.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let objects = solve1(program.clone());
    let answer1 = objects[2].len();
    let answer2 = solve2(objects.clone(), program.clone());
    println!("Day 13 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut program: Vec<i64>) -> Vec<HashSet<(usize, usize)>> {
    let mut computer = IntcodeComputer::new(0, program.clone(), 0);
    /**
     * 0 - empty tile
     * 1 - wall tile
     * 2 - block tile
     * 3 - horizontal paddle
     * 4 - ball tile
     */
    let mut objects = vec![HashSet::new(); 5];
    while !computer.has_halted {
        let x = computer.run_program();
        let y = computer.run_program();
        let obj = computer.run_program();
        objects[obj as usize].insert((x as usize, y as usize));
    }
    objects.clone()
}

fn solve2(mut objects: Vec<HashSet<(usize, usize)>>, mut program: Vec<i64>) -> i64 {
    program[0] = 2;
    let mut computer = IntcodeComputer::new(0, program.clone(), 0);
    let mut score = 0;
    let mut paddle_pos = objects[3].iter().nth(0).unwrap().clone();
    let mut ball_pos = objects[4].iter().nth(0).unwrap().clone();
    let height = objects[1]
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap()
        .1 as usize
        + 1;
    let width = objects[1]
        .iter()
        .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .unwrap()
        .0 as usize
        + 1;
    while !computer.has_halted {
        let x = computer.run_program();
        let y = computer.run_program();
        let val = computer.run_program();
        if x == -1 && y == 0 {
            score = val;
        } else {
            let new_pos = (x as usize, y as usize);
            if val == 3 {
                objects[3].clear();
                objects[3].insert(new_pos);
                paddle_pos = new_pos;
            } else if val == 4 {
                objects[4].clear();
                objects[4].insert(new_pos);
                ball_pos = new_pos;
            }
            if val != 2 {
                // Remove block tile if any
                objects[2].remove(&new_pos);
            }

            if paddle_pos.0 < ball_pos.0 {
                computer.input = 1;
            } else if paddle_pos.0 > ball_pos.0 {
                computer.input = -1;
            } else {
                computer.input = 0;
            }
        }
        draw_game(width, height, &objects, score);
    }
    draw_game(width, height, &objects, score);
    score
}

fn draw_game(width: usize, height: usize, objects: &Vec<HashSet<(usize, usize)>>, score: i64) {
    use std::{thread, time};
    let mut game_state = vec![vec![' '; width]; height];
    for (tile_type, obj) in objects.iter().enumerate() {
        for pos in obj {
            match tile_type {
                0 => game_state[pos.1][pos.0] = ' ',
                1 => game_state[pos.1][pos.0] = 'x',
                2 => game_state[pos.1][pos.0] = '#',
                3 => game_state[pos.1][pos.0] = '_',
                4 => game_state[pos.1][pos.0] = 'o',
                _ => (),
            }
        }
    }

    // Clear screen and reset
    print!("\x1B[2J\x1B[1;1H");
    println!(
        "{}\nScore: {}",
        game_state
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n"),
        score
    );
    let three_millis = time::Duration::from_millis(3);
    thread::sleep(three_millis);
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
