use core::num::ParseIntError;

#[derive(Debug)]
pub enum CommandParseError {
    InvalidOpcode(String),
    InvalidArg(ParseIntError),
}

impl From<ParseIntError> for CommandParseError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidArg(e)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Command {

}

impl TryFrom<(&str, &str)> for Command {
    type Error = CommandParseError;
    
    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let (opc, arg) = value;
        let arg: usize = arg.parse()?;

        Self::try_from((opc, arg))
    }
}

impl TryFrom<(&str, usize)> for Command {
    type Error = CommandParseError;

    fn try_from(value: (&str, usize)) -> Result<Self, Self::Error> {
        let r = match value {
            ("forward", arg) => Self::Forward(arg),
            ("down", arg) => Self::Down(arg),
            ("up", arg) => Self::Up(arg),
            (s, _) => return Err(CommandParseError::InvalidOpcode(s.to_owned()))
        };

        Ok(r)
    }
}

pub fn map_to_commands<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Command> {
    lines
        .filter(|l| !l.is_empty())    
        .map(str::split_whitespace)
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .map(Command::try_from)
        .map(Result::unwrap)
        .collect()
}

