use std::{cmp::max, fs, time::Instant};

use utils::day_input_path;

type Range = (u128, u128);

#[derive(Debug)]
pub struct InputRange {
    start: u128,
    start_str: String,
    end: u128,
    end_str: String,
}

// part 1 =============================================================
pub fn part1() {
    let start = Instant::now();
    let res: u128 = parse_ranges_from_file().into_iter().map(sum_in_range).sum();
    let duration_math = start.elapsed();
    println!("Result: {}, Time: {:?}", res, duration_math);
}

fn k_range(k: u32) -> Range {
    let start = 10u128.pow(k - 1);
    let end = start * 10 - 1;
    (start, end)
}

fn k(input: &InputRange) -> Range {
    let min_k = max(input.start_str.len().div_euclid(2) as u128, 1);
    let max_k = max(input.end_str.len().div_euclid(2) as u128, 1);
    (min_k, max_k)
}

fn sum_in_range(input: InputRange) -> u128 {
    let (mink, maxk) = k(&input);
    let mut total_sum = 0u128;
    for k in mink..=maxk {
        let kr = k_range(k as u32);
        let d: u128 = 10u128.pow(k as u32) + 1;
        let ss = (input.start).div_ceil(d);
        let se = input.end / d;
        let s_min = kr.0.max(ss);
        let s_max = kr.1.min(se);
        if s_min > s_max {
            continue;
        }
        for s in s_min..=s_max {
            let n = s * d;
            total_sum += n;
        }
    }
    total_sum
}

/// ## A valid N must satisfy
///
/// N = S * (10.pow(k*m) - 1) / (10.pow(k) - 1)
///
/// m = repetitions of S
///
/// k = digits of S
///
///
/// ## Example: 222
///
/// S = 2 | m = 3 | K = 1
///
/// N = 2 * 999/99  = 2 * 111 = 222
///
/// - Constraints
/// - Total digits L = k * m
///   for L_max: m must satisfy L mod m = 0
///
pub fn part2() {
    let start = Instant::now();
    let res: i128 = parse_ranges_from_file()
        .into_iter()
        .map(sum_in_range_2)
        .sum();
    let duration_math = start.elapsed();
    println!("Result: {}, Time: {:?}", res, duration_math);
}

fn sum_in_range_2(input: InputRange) -> i128 {
    let mut total_sum = 0i128;

    // k = number, m = repetitions
    // km gives all valid pairs of k and m for this range
    // for example, if lower value is 0 and upper is 9999,
    // this gives us:
    // (1, 2), (1, 3), (1, 4), (2, 2)
    for (k, m) in km(&input) {
        let mobius_factor = möbius(m);
        if mobius_factor == 0 {
            continue;
        }

        // exact k-digit S: 1
        // 10-99 for k = 2, 100-999 for k = 3, etc.
        let (k_min, k_max) = k_range(k as u32);

        // Constraint:
        // For given d in N = S * (10.pow(k*m) - 1) / (10.pow(k) - 1)
        // S => N / d, d = (10.pow(k*m) - 1) / (10.pow(k) - 1)
        // we know d, for start we calculate ceil, for end floor.
        // S <= start / d
        // S >= end / d
        let d = (10u128.pow((k * m) as u32) - 1) / (10u128.pow(k as u32) - 1);
        let s_min_range: u128 = input.start.div_ceil(d);
        let s_max_range: u128 = input.end / d;

        // Range overlap
        let s_min = s_min_range.max(k_min);
        let s_max = s_max_range.min(k_max);
        if s_min > s_max {
            continue;
        }

        // Arithmetic series sum
        // Formula: Sum(S) = (S_min + S_max) * Count / 2
        // Inclusive counting
        let count = s_max - s_min + 1;
        let as_sum_s = (s_min + s_max) * count / 2;

        // Calculate the total sum for this (k, m) pair: Sum(N) = Sum(S) * d
        let sum_n = as_sum_s * d;

        // Apply the corrected Möbius factor and accumulate to total_sum.
        let term_contribution: i128 = sum_n as i128 * mobius_factor;
        total_sum += term_contribution;
    }

    total_sum
}

fn km(input: &InputRange) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for l in input.start_str.len()..=input.end_str.len() {
        for m in divisors(l) {
            if m >= 2 {
                pairs.push((l / m, m));
            }
        }
    }
    pairs
}

fn möbius(mut m: usize) -> i128 {
    if m == 1 {
        return 1;
    }
    let mut p = 2;
    let mut sign = 1;
    while p * p <= m {
        // m is divisible by a square
        if m.is_multiple_of(p * p) {
            return 0;
        }
        // p is prime
        if m.is_multiple_of(p) {
            sign *= -1;
        }
        // reduce m until it's not divisible by p anymore
        while m.is_multiple_of(p) {
            m /= p;
        }
        p += 1;
    }
    // m is a prime itself: for initial m = 30, this is the p = 4, which is not a prime anymore,
    // and at this point is previously compared to p*p = 16 > m, for m = 5
    if m > 1 {
        sign *= -1;
    }
    -sign
}

fn parse_ranges_from_file() -> Vec<InputRange> {
    let input_path = day_input_path!();
    let content = fs::read_to_string(input_path).expect("Failed to read file");
    content
        .split(',')
        .filter_map(|part| {
            let mut nums = part.split('-');
            let start_str = nums.next()?.trim().to_owned();
            let end_str = nums.next()?.trim().to_owned();
            Some(InputRange {
                start: start_str.parse().ok()?,
                start_str,
                end: end_str.parse().ok()?,
                end_str,
            })
        })
        .collect()
}

fn divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    let mut i = 1;

    while i * i <= n {
        if n.is_multiple_of(i) {
            divs.push(i);

            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }

    // really don't need to be sorted
    divs.sort_unstable();
    divs
}
