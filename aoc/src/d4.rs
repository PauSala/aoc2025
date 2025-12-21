use utils::{day_input_path, read_lines};

static COLS: usize = 138;
static ROWS: usize = 138;

pub fn part1() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut input: Vec<u8> = Default::default();
        let mut result = 0;
        for line in lines.map_while(Result::ok) {
            for char in line.chars() {
                match char {
                    '.' => input.push(0),
                    _ => input.push(1),
                }
            }
        }
        for (i, v) in input.iter().enumerate() {
            if *v == 1 {
                let nb: u8 = neighbors(i).map(|i| input[i]).sum();
                if nb < 4 {
                    result += 1;
                }
            }
        }
        println!("{result}");
    }
}

pub fn part2() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut input: Vec<u8> = Default::default();
        let mut result = 0;
        for line in lines.map_while(Result::ok) {
            for char in line.chars() {
                match char {
                    '.' => input.push(0),
                    _ => input.push(1),
                }
            }
        }
        loop {
            let to_remove: Vec<usize> = input
                .iter()
                .enumerate()
                .filter(|(i, v)| **v == 1 && neighbors(*i).map(|idx| input[idx]).sum::<u8>() < 4)
                .map(|(i, _)| i)
                .collect();

            if to_remove.is_empty() {
                break;
            }

            for &index in &to_remove {
                input[index] = 0;
                result += 1;
            }
        }
        println!("{result}");
    }
}

static DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn neighbors(index: usize) -> impl Iterator<Item = usize> {
    let row = (index / COLS) as isize;
    let col = (index % COLS) as isize;
    DIRECTIONS.iter().filter_map(move |(drow, dcol)| {
        let new_row = row + drow;
        let new_col = col + dcol;
        if new_row >= 0 && new_col >= 0 && new_row < (ROWS as isize) && new_col < (COLS as isize) {
            Some((new_row as usize) * COLS + (new_col as usize))
        } else {
            None
        }
    })
}
