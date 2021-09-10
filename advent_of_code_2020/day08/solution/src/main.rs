mod machine;

use machine::{Instruction, Machine, Status};
use std::env;

fn main() {
    let fpath = &env::args().nth(1).unwrap_or("input.txt".to_string());
    let mut m = Machine::from(fpath).unwrap();

    m.set_debug(false).run();
    println!("Day8\n  Part1: {}", m.get_state().acc);

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

        /* swap the 2 instructions */
        let instr = m.ops.get(i + 1).unwrap(); // must be safe
        m.ops[i + 1] = match instr {
            Instruction::Jmp(v) => Instruction::Nop(*v),
            Instruction::Nop(v) => Instruction::Jmp(*v),
            a => panic!("owo {:?}", a),
        };
        i += 1;
    }
    println!("  Part2: {}", m.get_state().acc);
}
