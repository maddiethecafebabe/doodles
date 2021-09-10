mod instructions;
mod state;

pub use instructions::{Instruction, ParseIntError};
pub use state::State;
use std::{fs, mem};

#[derive(Debug)]
pub struct Machine {
    debug: bool,
    pub ops: Vec<Instruction>,
    state: Box<State>,
}

#[derive(Debug)]
pub enum Status {
    Running(Option<State>),
    Finished(Option<State>),
    Inflooped(Option<State>),
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            debug: false,
            ops: vec![],
            state: State::new(),
        }
    }

    pub fn from(fpath: &String) -> Result<Self, ParseIntError> {
        let mut new = Machine::new();
        let raw = match fs::read_to_string(fpath) {
            Ok(r) => r,
            Err(e) => {
                panic!("Failed to read file '{}': {:?}", fpath, e)
            }
        };
        match (&mut new).load_instructions(&raw) {
            Ok(_) => Ok(new),
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn set_debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    pub fn load_instructions(&mut self, raw: &String) -> Result<&mut Self, ParseIntError> {
        self.ops = raw
            .split("\n")
            .filter(|l| !l.is_empty())
            .map(|l| Instruction::from_string(l).unwrap())
            .collect::<Vec<Instruction>>();
        Ok(self)
    }

    pub fn do_single_instruction(&mut self, instr: &Instruction) -> Status {
        self.state.history.push(self.state.pc);
        match instr {
            Instruction::Acc(i) => {
                self.state.inc_acc(*i);
                self.state.inc_pc(1);
            }
            Instruction::Jmp(i) => {
                self.state.inc_pc(*i);
            }
            Instruction::Nop(_) => {
                self.state.inc_pc(1);
            }
            Instruction::Unknown => {
                println!("[?] @{} Hit unknown instruction'", self.state.pc);
            }
        };

        self.get_status()
    }

    pub fn fetch_next_instruction(&mut self) -> Instruction {
        let i = self.ops.get(self.state.pc as usize).unwrap();
        //println!("{:?}", i);
        *i
    }

    pub fn step(&mut self, cnt: usize) -> Status {
        for _ in 0..cnt {
            let instr = self.fetch_next_instruction();
            self.do_single_instruction(&instr);
        }
        self.get_status()
    }

    pub fn get_status(&self) -> Status {
        let s = match self.debug {
            true => Some(*self.state.clone()),
            false => None,
        };

        if self.state.history.contains(&self.state.pc) {
            return Status::Inflooped(s);
        } else if self.state.pc >= self.ops.len() {
            return Status::Finished(s);
        }
        Status::Running(s)
    }

    pub fn run(&mut self) -> &mut Self {
        while match self.step(1) {
            Status::Running(_) => true,
            _ => false,
        } {}
        self
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &State {
        &*self.state
    }

    #[allow(dead_code)]
    pub fn set_state(&mut self, new: State) -> &mut Self {
        let _ = mem::replace(&mut *self.state, new);
        self
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) -> &mut Self {
        self.state.clear();
        self
    }
}

impl Iterator for Machine {
    type Item = Status;
    fn next(&mut self) -> Option<Status> {
        let instr = self.fetch_next_instruction();

        match self.do_single_instruction(&instr) {
            Status::Running(state) => Some(Status::Running(state)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_constructors() {
        let _m1 = Machine::new();
    }

    #[test]
    fn part1_follow_simple_opcodes() {
        let input =
            "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6".to_string();

        let mut m = Machine::new();
        m.load_instructions(&input).unwrap().set_debug(true);
        m.run();

        if let Status::Inflooped(_) = m.get_status() {
        } else {
            panic!("Did not infloop");
        }
        assert_eq!(m.get_state().acc, 5);
        assert_eq!(m.get_state().pc, 1);

        //println!("{:?}", m.get_status());
    }

    #[test]
    fn part2_fix_infloop() {
        let input =
            "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6".to_string();

        let mut m = Machine::new();
        m.load_instructions(&input).unwrap().set_debug(true);

        let mut i = 0;
        let old = m.ops.clone();
        while let Status::Inflooped(_) = m.run().get_status() {
            m.reset();
            m.ops = old.clone();

            let i_ = m
                .ops
                .iter()
                .skip(i + 1)
                .collect::<Vec<&Instruction>>()
                .iter()
                .position(|instr| match instr {
                    Instruction::Jmp(_) => true,
                    Instruction::Nop(_) => true,
                    _ => false,
                });
            i += match i_ {
                Some(v) => v,
                None => {
                    panic!("couldnt find fucker");
                }
            };

            let instr = m.ops.get(i + 1).unwrap(); // must be safe
            m.ops[i + 1] = match instr {
                Instruction::Jmp(v) => Instruction::Nop(*v),
                Instruction::Nop(v) => Instruction::Jmp(*v),
                a => panic!("owo {:?}", a),
            };
            i += 1;
        }
        assert_eq!(m.get_state().acc, 8);
    }
}
