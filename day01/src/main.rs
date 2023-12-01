#![feature(thread_id_value)]
#![feature(exclusive_range_pattern)]

use std::time::Duration;
use timeloop::RepititionTester;

timeloop::impl_enum!(
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Timer {
        Part1,
        Part2,
    }
);

timeloop::create_profiler!(Timer);

fn part1(data: &str) -> u32 {
    timeloop::scoped_timer!(Timer::Part1);

    let mut sum = 0_u32;

    for line in data.lines() {
        let mut first = None;
        let mut last = None;

        for byte in line.bytes() {
            if byte >= b'0' && byte <= b'9' {
                let num = byte - b'0';
                first = first.or(Some(num));
                last = Some(num);
            }
        }

        sum += (first.unwrap() * 10 + last.unwrap()) as u32;
    }

    sum
}

fn part2(data: &str) -> u32 {
    timeloop::scoped_timer!(Timer::Part2);
    let mut sum = 0_u32;

    for line in data.lines() {
        let mut first = None;
        let mut last = 0;

        let bytes = line.as_bytes();
        let mut i = 0;
        loop {
            if i >= line.len() {
                break;
            }

            let num = match bytes[i..] {
                [b'0'..=b'9', ..] => {
                    let res = bytes[i] - b'0';
                    res
                }
                [b'o', b'n', b'e', ..] => 1,
                [b't', b'w', b'o', ..] => 2,
                [b't', b'h', b'r', b'e', b'e', ..] => 3,
                [b'f', b'o', b'u', b'r', ..] => 4,
                [b'f', b'i', b'v', b'e', ..] => 5,
                [b's', b'i', b'x', ..] => 6,
                [b's', b'e', b'v', b'e', b'n', ..] => 7,
                [b'e', b'i', b'g', b'h', b't', ..] => 8,
                [b'n', b'i', b'n', b'e', ..] => 9,
                _ => {
                    i += 1;
                    continue;
                }
            };

            i += 1;
            first = first.or(Some(num));
            last = num;
        }

        sum += (first.unwrap() * 10 + last) as u32;
    }

    sum
}

// The functions being tested
const FUNCS: &[(&'static str, fn(&str) -> u32)] = &[("Part1", part1), ("Part2", part2)];

fn main() {
    let data = include_str!("../input");

    timeloop::start_profiler!();

    // Run the functions to get the default answers
    for func in FUNCS.iter() {
        let res = func.1(data);
        println!("{}: {}", func.0, res);
    }

    // Run the repeated benchmarks
    for _ in 0..3 {
        for func in FUNCS.iter() {
            let mut tester = RepititionTester::new(Duration::from_secs(2));

            while tester.is_testing() {
                // Start the timer for this iteration
                tester.start();

                // Execute the function in question
                let _result = func.1(&data);

                // Stop the timer for this iteration
                tester.stop();
            }

            println!("----- {} -----", func.0);
            tester.results.print_with_bytes(data.len() as u64);
        }
    }

    // Print the stats
    timeloop::print!();
}
