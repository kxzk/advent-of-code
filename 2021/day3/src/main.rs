use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // since it'll be const for this
    let mut binary_values: Vec<i16> = vec![0; 12];

    for line in reader.lines() {
        let line = line?;

        for (k, v) in line.chars().enumerate() {
            match v {
                '0' => binary_values[k] -= 1,
                '1' => binary_values[k] += 1,
                _ => panic!("Unexpected character"),
            }
        }
    }

    println!("binary_values={:?}", binary_values);

    let gamma_rate: String = binary_values
        .iter()
        .map(|&x| if x > 0 { 1 as u8 } else { 0 as u8 })
        .map(|x| (x + 48) as char)
        .collect();
    println!("gamm_rate={:?}", gamma_rate);
    let epsilon_rate: String = gamma_rate
        .chars()
        .map(|x| if x == '1' { "0" } else { "1" })
        .collect();
    println!("epsilon_rate={:?}", epsilon_rate);

    let gamma_rate_bin = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_bin = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    println!(
        "gamma_rate_bin={}, epsilon_rate_bin={}",
        gamma_rate_bin, epsilon_rate_bin
    );

    let result = gamma_rate_bin * epsilon_rate_bin;
    println!("result={}", result);

    Ok(())
}
