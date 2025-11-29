use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;

    let mut level: i32 = 0;
    let mut index: u64 = 1;
    let mut basement_found = false;

    for c in input.chars() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            _ => (),
        }
        if level < 0 && !basement_found {
            println!("first_basement_index={}", index);
            basement_found = true;
        }
        index += 1;
    }

    println!("final_level={}", level);

    Ok(())
}
