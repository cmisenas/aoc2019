#[derive(Debug)]
pub struct IntcodeComputer {
    pub id: usize,
    pub program: Vec<i64>,
    pub memory: Vec<i64>,
    pub input: i64,
    pub index: usize,
    pub has_halted: bool,
    pub diagnostics_code: i64,
    pub use_phase: bool,
    pub relative_base: i64,
}

impl IntcodeComputer {
    pub fn new(id: usize, program: Vec<i64>, input: i64) -> IntcodeComputer {
        IntcodeComputer {
            id,
            program,
            input,
            memory: Vec::new(),
            index: 0,
            has_halted: false,
            use_phase: true,
            diagnostics_code: 0,
            relative_base: 0,
        }
    }

    pub fn run_program(&mut self) -> i64 {
        loop {
            let opcode = self.program[self.index] % 100;
            let mode1 = (self.program[self.index] / 100) % 10;
            let mode2 = (self.program[self.index] / 1000) % 10;
            let mode3 = (self.program[self.index] / 10000) % 10;
            match opcode {
                1 | 2 | 7 | 8 => {
                    let param1 = self.get_addr(mode1, self.index + 1);
                    let param2 = self.get_addr(mode2, self.index + 2);
                    let less_than = opcode == 7 && param1 < param2;
                    let equal = opcode == 8 && param1 == param2;
                    let result = match opcode {
                        1 => param1 + param2,
                        2 => param1 * param2,
                        _ => 0,
                    };
                    if less_than || equal {
                        self.set_addr(mode3, self.index + 3, 1);
                    } else {
                        self.set_addr(mode3, self.index + 3, result);
                    }
                    self.index += 4;
                }
                3 | 4 => {
                    match opcode {
                        // Input the value
                        3 => {
                            self.set_addr(mode1, self.index + 1, self.input);
                            self.use_phase = false;
                            self.index += 2;
                        }
                        // Output the value
                        _ => {
                            let param1 = self.get_addr(mode1, self.index + 1);
                            self.diagnostics_code = param1;
                            self.index += 2;
                            return self.diagnostics_code;
                        }
                    };
                }
                5 | 6 => {
                    let param1 = self.get_addr(mode1, self.index + 1);
                    let param2 = self.get_addr(mode2, self.index + 2);
                    let jump_if_true = opcode == 5 && param1 != 0;
                    let jump_if_false = opcode == 6 && param1 == 0;
                    if jump_if_true || jump_if_false {
                        self.index = param2 as usize;
                    } else {
                        self.index += 3;
                    }
                }
                9 => {
                    let param1 = self.get_addr(mode1, self.index + 1);
                    self.relative_base += param1;
                    self.index += 2;
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

    pub fn get_addr(&mut self, mode: i64, val: usize) -> i64 {
        let addr = match mode {
            0 => self.program[val] as usize,
            1 => val,
            2 => (self.relative_base + self.program[val]) as usize,
            _ => panic!("UNKNOWN MODE {}", mode),
        };
        if addr >= self.program.len() {
            let mem_addr = addr % self.program.len();
            while mem_addr >= self.memory.len() {
                self.memory.push(0);
            }
            self.memory[mem_addr]
        } else {
            self.program[addr]
        }
    }

    // No mode necessary. Setting is always position mode.
    pub fn set_addr(&mut self, mode: i64, addr: usize, val: i64) {
        let final_addr = match mode {
            0 => self.program[addr] as usize,
            2 => (self.relative_base + self.program[addr]) as usize,
            _ => panic!("UNKNOWN MODE {}", mode),
        };
        if final_addr >= self.program.len() {
            let mem_addr = final_addr % self.program.len();
            while mem_addr >= self.memory.len() {
                self.memory.push(0);
            }
            self.memory[mem_addr] = val;
        } else {
            self.program[final_addr] = val;
        }
    }
}
