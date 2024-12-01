use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Debugger {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<i8>,
    instruction_ptr: usize,
}

impl FromStr for Debugger {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();
        let reg_a = u64::from_str(lines[0].split(": ").collect::<Vec<_>>()[1])?;
        let reg_b = u64::from_str(lines[1].split(": ").collect::<Vec<_>>()[1])?;
        let reg_c = u64::from_str(lines[2].split(": ").collect::<Vec<_>>()[1])?;
        let program_str = lines[4].split(": ").collect::<Vec<_>>()[1];
        let program: Vec<_> = program_str.split(",").filter_map(|s| i8::from_str(s).ok()).collect();
        Ok(Self { reg_a, reg_b, reg_c, program, instruction_ptr: 0 })
    }
}

impl Debugger {
    pub fn run(&mut self) -> Vec<u64> {
        let mut results: Vec<u64> = vec![];
        while self.instruction_ptr < self.program.len() - 1 {
            let instruction = self.program[self.instruction_ptr];
            let operand = self.program[self.instruction_ptr + 1];
            let (jump, result) = self.calculate(instruction, operand);

            if let Some(num) = result {
                results.push(num)
            }
            if !jump {
                self.instruction_ptr += 2;
            }
        }
        results
    }

    pub fn reverse(&self) -> u64 {
        0
    }

    fn calculate(&mut self, instruction: i8, operand: i8) -> (bool, Option<u64>) {
        if instruction == 0 {                                       // adv
            self.reg_a /= 2_u64.pow(self.combo_operand(operand) as u32);
            (false, None)
        } else if instruction == 1 {                                // bxl
            self.reg_b ^= operand as u64;
            (false, None)
        } else if instruction == 2 {                                // bst
            self.reg_b = self.combo_operand(operand) % 8;
            (false, None)
        } else if instruction == 3 && self.reg_a != 0 {             // jnz
            self.instruction_ptr = operand as usize;
            (true, None)
        } else if instruction == 4 {                                // bxc
            self.reg_b ^= self.reg_c;
            (false, None)
        } else if instruction == 5 {                                // out
            (false, Some(self.combo_operand(operand) % 8))
        } else if instruction == 6 {                                // bdv
            self.reg_b = self.reg_a / 2_u64.pow(self.combo_operand(operand) as u32);
            (false, None)
        } else if instruction == 7 {                                // cdv
            self.reg_c = self.reg_a / 2_u64.pow(self.combo_operand(operand) as u32);
            (false, None)
        } else {
            (false, None)
        }
    }

    #[inline]
    fn combo_operand(&self, operand: i8) -> u64 {
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => operand as u64
        }
    }
}
