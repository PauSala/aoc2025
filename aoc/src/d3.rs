use std::time::Instant;

use utils::{day_input_path, read_lines};

pub fn part1() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let mut sum: u32 = 0;
        for line in lines.map_while(Result::ok) {
            let nums: Vec<u8> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            let max_left: Vec<u8> = nums
                .iter()
                .scan(0u8, |m, &x| {
                    *m = (*m).max(x);
                    Some(*m)
                })
                .collect();

            let mut max_right: Vec<u8> = nums
                .iter()
                .rev()
                .scan(0u8, |m, &x| {
                    let out = *m;
                    *m = (*m).max(x);
                    Some(out)
                })
                .collect();
            max_right.reverse();

            let mut best = 0u32;
            let end = nums.len().saturating_sub(1);
            for i in 0..end {
                let val = (max_left[i] as u32) * 10 + (max_right[i] as u32);
                if val > best {
                    best = val;
                }
            }
            sum += best;
        }

        println!("Result: {sum}");
    }
}

// Monotonic stack
pub fn part2() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let start = Instant::now();
        let mut res: u64 = 0;
        let seq_len = 12;
        for line in lines.map_while(Result::ok) {
            let input: Vec<String> = line.chars().map(|c| c.to_string()).collect();
            let mut stack: Vec<String> = Vec::new();

            for (i, num) in input.iter().enumerate() {
                let remaining = input.len() - i;

                while let Some(last) = stack.last() {
                    if last < num && stack.len() - 1 + remaining >= seq_len {
                        stack.pop();
                    } else {
                        break;
                    }
                }

                if stack.len() < seq_len {
                    stack.push(num.to_string());
                }
            }

            res += stack
                .iter()
                .fold("".to_string(), |mut acc, c| {
                    acc.push_str(c);
                    acc
                })
                .parse::<u64>()
                .unwrap();
        }
        let duration_math = start.elapsed();
        println!("Result: {}, Time: {:?}", res, duration_math);
    }
}

// DP
pub fn part_2() {
    if let Ok(lines) = read_lines(day_input_path!()) {
        let start = Instant::now();
        let mut res: u64 = 0;
        const N: usize = 100;
        let seq_len = 12;
        for line in lines.map_while(Result::ok) {
            let input: Vec<String> = line.chars().map(|c| c.to_string()).collect();
            let mut cache: [[String; N]; N] =
                std::array::from_fn(|_| std::array::from_fn(|_| String::new()));
            let mut best = String::new();
            for i in (0..input.len()).rev() {
                best = best.max(input[i].clone());
                cache[i][0] = best.clone();
            }
            max_seq(seq_len, &input, &mut cache);
            let max = cache
                .iter()
                .map(|f| f[seq_len - 1].parse::<u64>().unwrap_or(0))
                .max()
                .unwrap();
            res += max;
        }
        let duration_math = start.elapsed();
        println!("Result: {}, Time: {:?}", res, duration_math);
    }
}

// max_seq(n, i) = max(a[i] <> V(n−1, i+1) , V(n, i+1))
fn max_seq<const N: usize>(max_seq_len: usize, arr: &[String], cache: &mut [[String; N]]) {
    // We already have V(1, i) in cache[i][0]
    for seq_len in 2..=max_seq_len {
        let col = seq_len - 1;

        for i in (0..arr.len()).rev() {
            // TAKE: a[i] <> V(len-1, i+1)
            let take = if i + seq_len <= arr.len() {
                let mut s = arr[i].clone();
                s.push_str(&cache[i + 1][col - 1]);
                s
            } else {
                String::new()
            };

            // SKIP: V(len, i+1)
            let skip = if i + 1 < arr.len() {
                cache[i + 1][col].clone()
            } else {
                String::new()
            };
            // MAX
            cache[i][col] = if take > skip { take } else { skip };
        }
    }
}
