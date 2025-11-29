use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let width = input.lines().next().unwrap().len();

    let mut counts: Vec<i32> = vec![0, width];

    for line in input.lines() {
        for (i, c) in line.bytes().enumerate() {
            counts[i] += match c {
                b'0' => -1,
                b'1' => 1,
                _ => panic!("unexpected_char={}", c as char),
            };
        }
    }

    let gamma: u32 = counts.iter().fold(0, |acc, &c| (acc << 1) | (c > 0) as u32);

    let epsilon = !gamma & ((1 << width) - 1);

    println!("result={}", gamma * epsilon);
    Ok(())
}
