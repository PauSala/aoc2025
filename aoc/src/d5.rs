use std::ops::Range;
use std::ops::RangeInclusive;

use utils::{day_input_path, read_lines};

pub fn part1() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut mode = 0;
        let mut ranges: Vec<RangeInclusive<usize>> = Default::default();
        let mut values: Vec<u64> = Default::default();
        let mut result = 0;
        for line in lines.map_while(Result::ok) {
            if line == "" {
                mode = 1;
                continue;
            }
            match mode {
                0 => {
                    let split = line.split('-');
                    let nums: Vec<usize> = split
                        .into_iter()
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect();
                    ranges.push(nums[0]..=nums[1]);
                }
                _ => {
                    values.push(line.parse().unwrap());
                }
            }
        }
        for v in values {
            for range in &ranges {
                if range.contains(&(v as usize)) {
                    result += 1;
                    break;
                }
            }
        }
        println!("{result}");
    }
}

pub fn part2() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut mode = 0;
        let mut ranges: Vec<Range<usize>> = Default::default();
        for line in lines.map_while(Result::ok) {
            if line == "" {
                mode = 1;
                continue;
            }
            match mode {
                0 => {
                    let split = line.split('-');
                    let nums: Vec<usize> = split
                        .into_iter()
                        .map(|e| e.parse::<usize>().unwrap())
                        .collect();
                    ranges.push(nums[0]..nums[1]);
                }
                _ => {}
            }
        }
        let merged = merge_ranges(ranges);
        let mut count = 0;
        for range in merged {
            count += (range.end - range.start) + 1;
        }
        println!("Result: {count}");
    }
}

fn merge_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.start);
    let mut merged = vec![ranges[0].clone()];
    for range in ranges.into_iter().skip(1) {
        if let Some(last) = merged.last_mut() {
            if last.end >= range.start {
                last.end = last.end.max(range.end)
            } else {
                merged.push(range);
            }
        }
    }
    merged
}
