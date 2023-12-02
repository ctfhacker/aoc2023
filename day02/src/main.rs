#![feature(thread_id_value)]
#![feature(exclusive_range_pattern)]

use anyhow::Result;
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

fn part1(data: &str) -> Option<u32> {
    timeloop::scoped_timer!(Timer::Part1);

    let mut sum = 0_u32;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        // Parse out Game <ID>
        let mut line_iter = line.split(": ");
        let id = line_iter
            .next()?
            .split(" ")
            .skip(1)
            .next()?
            .parse::<u32>()
            .ok()?;

        let games = line_iter.next()?.replace(";", ",");
        // println!("{id} {games}");

        let games = games.split(", ");
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for g in games {
            let num = g.split(" ").next()?.parse::<u32>().ok()?;

            if g.contains(&"red") {
                max_red = max_red.max(num);
            } else if g.contains(&"green") {
                max_green = max_green.max(num);
            } else if g.contains(&"blue") {
                max_blue = max_blue.max(num);
            }
        }

        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum += id;
        }
    }

    Some(sum)
}

fn part2(data: &str) -> Option<u32> {
    timeloop::scoped_timer!(Timer::Part2);

    let mut sum = 0_u32;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        // Parse out Game <ID>
        let mut line_iter = line.split(": ");
        let _id = line_iter
            .next()?
            .split(" ")
            .skip(1)
            .next()?
            .parse::<u32>()
            .ok()?;

        let games = line_iter.next()?.replace(";", ",");
        // println!("{id} {games}");

        let games = games.split(", ");
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;
        for g in games {
            let num = g.split(" ").next()?.parse::<u32>().ok()?;

            if g.contains(&"red") {
                // red
                max_red = max_red.max(num);
            } else if g.contains(&"green") {
                max_green = max_green.max(num);
            } else if g.contains(&"blue") {
                max_blue = max_blue.max(num);
            }
        }

        sum += max_red * max_blue * max_green;
    }

    Some(sum)
}

fn part2_opt1(data: &str) -> Option<u32> {
    timeloop::scoped_timer!(Timer::Part2);

    let mut sum = 0_u32;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        let line = line.as_bytes();
        let mut i = "Game ".len();

        // Parse past the Game <ID>: header
        loop {
            if line[i] == b':' {
                i += 2;
                break;
            }

            i += 1;
        }

        // Parse the next game
        loop {
            if i >= line.len() {
                break;
            }

            // Parse the number
            let num = match line[i] {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                _ => unreachable!(),
            };
            i += 1;

            // Parse the potential two digit
            let (num, offset) = match line[i] {
                b'0' => (num * 10 + 0, 2),
                b'1' => (num * 10 + 1, 2),
                b'2' => (num * 10 + 2, 2),
                b'3' => (num * 10 + 3, 2),
                b'4' => (num * 10 + 4, 2),
                b'5' => (num * 10 + 5, 2),
                b'6' => (num * 10 + 6, 2),
                b'7' => (num * 10 + 7, 2),
                b'8' => (num * 10 + 8, 2),
                b'9' => (num * 10 + 9, 2),
                b' ' => (num, 1),
                _ => unreachable!(),
            };
            i += offset;

            // Parse the color based on the first letter
            match line[i] {
                b'r' => {
                    max_red = max_red.max(num);
                    i += "red; ".len();
                }
                b'b' => {
                    max_blue = max_blue.max(num);
                    i += "blue; ".len();
                }
                b'g' => {
                    max_green = max_green.max(num);
                    i += "green; ".len();
                }
                _ => unreachable!(),
            }
        }

        // Add the answer
        sum += max_red * max_blue * max_green;
    }

    Some(sum)
}

// The functions being tested
const FUNCS: &[(&'static str, fn(&str) -> Option<u32>)] = &[
    ("Part1", part1),
    ("Part2", part2),
    ("Part2 Opt1", part2_opt1),
];

fn main() {
    let data = include_str!("../input");

    timeloop::start_profiler!();

    // Run the functions to get the default answers
    for func in FUNCS.iter() {
        let res = func.1(data);
        println!("{}: {:?}", func.0, res);
    }

    // Run the repeated benchmarks
    for _ in 0..3 {
        for func in FUNCS.iter() {
            let mut tester = RepititionTester::new(Duration::from_secs(4));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(data).unwrap();
        assert_eq!(result, 8);

        let result = part2(data).unwrap();
        assert_eq!(result, 2286);
    }
}
