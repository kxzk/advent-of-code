use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut increase_count: u16 = 0;

    let mut previous_line: Option<u16> = None;

    for line in reader.lines() {
        let line = line?;
        let current_value: u16 = line.trim().parse()?;

        if previous_line.is_some_and(|prev| current_value > prev) {
            increase_count += 1;
        }
        previous_line = Some(current_value);
    }

    println!("increases={}", increase_count);

    Ok(())
}
