use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// forward u8
// down u8
// up u8
enum Command {
    Forward(u8),
    Down(u8),
    Up(u8),
}

#[derive(Debug)]
struct ParseCommandError;

impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse command")
    }
}

impl std::error::Error for ParseCommandError {}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, arg) = s.split_once(' ').ok_or(ParseCommandError)?;
        let value = arg.parse().map_err(|_| ParseCommandError)?;

        match cmd {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(ParseCommandError),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal: u16 = 0;
    let mut depth: u16 = 0;

    let final_result: u32;

    for line in reader.lines() {
        let line = line?;
        let command: Command = line.parse()?;

        match command {
            Command::Forward(x) => horizontal += x as u16,
            Command::Down(x) => depth += x as u16,
            Command::Up(x) => depth -= x as u16,
        }
    }

    final_result = horizontal as u32 * depth as u32;
    println!("final_result={}", final_result);

    Ok(())
}
