use utils::{day_input_path, read_lines};

pub fn part1() {
    let start = 50isize;
    let modulus = 100isize;
    let mut count = 0;
    let mut pos = start;

    if let Ok(lines) = read_lines(day_input_path!()) {
        for line in lines.map_while(Result::ok) {
            let (dir, rest) = line.split_at(1);
            let n: isize = rest.trim().parse().unwrap();

            let delta = match dir {
                "L" => -n,
                "R" => n,
                _ => continue,
            };
            pos = (pos + delta) % modulus;
            if pos == 0 {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}

pub fn part2() {
    let start = 50isize;
    let modulus = 100isize;
    let mut count = 0;
    let mut pos = start;

    if let Ok(lines) = read_lines(day_input_path!()) {
        for line in lines.map_while(Result::ok) {
            let (dir, rest) = line.split_at(1);
            let n: isize = rest.trim().parse().unwrap();

            let delta = match dir {
                "L" => -n,
                "R" => n,
                _ => continue,
            };

            let total = pos + delta;
            let wraps = total.div_euclid(modulus) - pos.div_euclid(modulus);

            count += wraps.abs();
            pos = total.rem_euclid(modulus);
        }
    }

    println!("Result: {}", count);
}
