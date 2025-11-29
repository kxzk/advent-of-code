// 2lw + 2wh + 2hl
// min of lw, wh, hl
// l -> length
// w -> width
// h -> height
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;

    let mut area: usize = 0;

    for line in input.lines() {
        let dims: [u8; 3] = line
            .split('x')
            .map(|x| x.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<_>| format!("expected 3 dimensions, got {}", v.len()))?;

        let [l, w, h] = dims;

        let lw = (l as usize) * (w as usize);
        let wh = (w as usize) * (h as usize);
        let hl = (h as usize) * (l as usize);

        let min_val = [lw, wh, hl].into_iter().min().unwrap();

        area += 2 * lw + 2 * wh + 2 * hl + min_val;
    }

    println!("total_area={}", area);

    Ok(())
}
