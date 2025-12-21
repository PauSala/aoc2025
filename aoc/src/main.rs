use std::env;

pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }
    let day: u32 = args[2].parse().expect("Day must be a number");

    match day {
        1 => {
            d1::part1();
            d1::part2();
        }
        2 => {
            d2::part1();
            d2::part2();
        }
        3 => {
            d1::part1();
            d3::part2();
        }
        4 => {
            d4::part1();
            d4::part2();
        }
        5 => {
            d5::part1();
            d5::part2();
        }
        6 => {
            d6::part1();
            d6::part2();
        }
        _ => eprintln!("Day {} not implemented", day),
    }
}
