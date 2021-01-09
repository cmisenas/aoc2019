use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct AmplifierController {
    id: usize,
    program: Vec<i32>,
    phase_setting: i32,
    input: i32,
    index: usize,
    has_halted: bool,
    diagnostics_code: i32,
    use_phase: bool,
}

impl AmplifierController {
    fn new(id: usize, program: Vec<i32>, phase_setting: i32, input: i32) -> AmplifierController {
        AmplifierController {
            id,
            program,
            phase_setting,
            input,
            index: 0,
            has_halted: false,
            use_phase: true,
            diagnostics_code: 0,
        }
    }

    // Returns an output when output instruction encountered or halted
    // Need to use has_halted bool to check if halted true
    fn run_program(&mut self) -> i32 {
        loop {
            let opcode = self.program[self.index] % 100;
            let mode1 = (self.program[self.index] / 100) % 10;
            let mode2 = self.program[self.index] / 1000;
            match opcode {
                1 | 2 | 7 | 8 => {
                    let param1 = match mode1 {
                        1 => self.program[self.index + 1],
                        _ => self.program[self.program[self.index + 1] as usize],
                    };
                    let param2 = match mode2 {
                        1 => self.program[self.index + 2],
                        _ => self.program[self.program[self.index + 2] as usize],
                    };
                    let store = self.program[self.index + 3] as usize; // ALWAYS POSITION MODE
                    let less_than = opcode == 7 && param1 < param2;
                    let equal = opcode == 8 && param1 == param2;
                    let result = match opcode {
                        1 => param1 + param2,
                        2 => param1 * param2,
                        _ => 0,
                    };
                    if less_than || equal {
                        self.program[store] = 1;
                    } else {
                        self.program[store] = result;
                    }
                    self.index += 4;
                }
                3 | 4 => {
                    let addr = self.program[self.index + 1] as usize;
                    match opcode {
                        // Input the value
                        3 => {
                            self.program[addr] = match self.use_phase {
                                true => self.phase_setting,
                                false => self.input,
                            };
                            self.use_phase = false;
                            self.index += 2;
                        }
                        // Output the value
                        _ => {
                            self.diagnostics_code = self.program[addr];
                            self.index += 2;
                            return self.diagnostics_code;
                        }
                    };
                }
                5 | 6 => {
                    let param1 = match mode1 {
                        1 => self.program[self.index + 1],
                        _ => self.program[self.program[self.index + 1] as usize],
                    };
                    let param2 = match mode2 {
                        1 => self.program[self.index + 2],
                        _ => self.program[self.program[self.index + 2] as usize],
                    };
                    let jump_if_true = opcode == 5 && param1 != 0;
                    let jump_if_false = opcode == 6 && param1 == 0;
                    if jump_if_true || jump_if_false {
                        self.index = param2 as usize;
                    } else {
                        self.index += 3;
                    }
                }
                99 => {
                    self.has_halted = true;
                    break;
                }
                _ => {
                    panic!("Unknown opcode {}", opcode);
                    break;
                }
            };
        }
        self.diagnostics_code
    }
}

pub fn main() {
    let lines = read_lines_as_str("./day7.input");
    let program = lines[0]
        .split(",")
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("Day 7 answers");
    let answer1 = solve1(program.clone());
    let answer2 = solve2(program.clone());
    println!("Answer 1 {} from seq {}", answer1.1, answer1.0);
    println!("Answer 2 {} from seq {}", answer2.1, answer2.0);
}

fn solve1(program: Vec<i32>) -> (String, i32) {
    let possible_seq = (0..=4).permutations(5);
    let mut results: HashMap<String, i32> = HashMap::new();
    let init = 0;
    for seq in possible_seq.into_iter() {
        let mut result = init;
        for phase_setting in seq.iter() {
            let mut module = AmplifierController::new(0, program.clone(), *phase_setting, result);
            result = module.run_program();
        }
        results.insert(seq.iter().map(|s| s.to_string()).join(""), result);
    }
    let result = results.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    (result.0.to_string(), *result.1)
}

fn solve2(mut program: Vec<i32>) -> (String, i32) {
    let second_half = (5..=9).permutations(5);
    let mut results: HashMap<String, i32> = HashMap::new();
    let init = 0;
    for (seq_i, seq2) in second_half.into_iter().enumerate() {
        let mut result = init;
        let mut seq_id = String::from("");
        let mut modules = Vec::new();
        for (i, phase_setting) in seq2.iter().enumerate() {
            let mut module = AmplifierController::new(i, program.clone(), *phase_setting, result);
            result = module.run_program();
            modules.push(module);
            seq_id.push_str(&phase_setting.to_string());
        }
        let mut module_i = 0;
        while !modules[4].has_halted {
            modules[module_i].input = result;
            result = modules[module_i].run_program();
            module_i = (module_i + 1) % 5;
        }
        result = modules[4].run_program();
        results.insert(seq_id, result);
    }
    let result = results.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    (result.0.to_string(), *result.1)
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
