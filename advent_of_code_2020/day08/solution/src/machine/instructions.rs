pub use core::num::ParseIntError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
    Unknown,
}

impl Instruction {
    pub fn from_string(string: &str) -> Result<Instruction, ParseIntError> {
        let (op, arg) = string
            .trim()
            .split_at(string.trim().find(' ').expect("Malformed input line"));

        let (op, arg) = match (op, arg.trim().parse::<i32>()) {
            (_, Ok(arg)) => (op, arg),
            (_, Err(e)) => {
                panic!("Failed to parse '{}' to an i32: {:?}", arg.trim(), e);
            }
        };

        let r = match (op, arg) {
            ("acc", a) => Instruction::Acc(a),
            ("jmp", a) => Instruction::Jmp(a),
            ("nop", a) => Instruction::Nop(a),
            (l, _) => {
                println!("Unrecognized Instruction: {:?}", l);
                Instruction::Unknown
            }
        };
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_string_parsing() {
        assert_eq!(
            Instruction::from_string("acc 2 ").unwrap(),
            Instruction::Acc(2)
        );
        assert_eq!(
            Instruction::from_string("   acc -9 ").unwrap(),
            Instruction::Acc(-9)
        );
        assert_eq!(
            Instruction::from_string("  nop 2  ").unwrap(),
            Instruction::Nop(2)
        );
        assert_eq!(
            Instruction::from_string(" jmp 0").unwrap(),
            Instruction::Jmp(0)
        );
    }
}
