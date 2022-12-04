#![feature(test)]
extern crate test;
use std::fs;

fn main() {
    let path = "day2input.txt";
    let input = fs::read_to_string(path);
    let input = match input {
        Ok(s) => s,
        Err(_) => panic!("input file not found"),
    };

    let output = part1(&input[..]);
    println!("{}", output);
    let output = part2(&input[..]);
    println!("{}", output);
}

fn part1(input: &str) -> i32 {
    let input: Vec<&str> = input.split("\n").collect();
    let mut total_score = 0;

    for i in input {
        if i.len() < 3 {
            continue;
        }
        let mut chars = i.chars();
        let first = chars.next().unwrap();
        chars.next();
        let second = chars.next().unwrap();

        let outcome = match first {
            'A' => match second {
                'X' => 3,
                'Y' => 6,
                'Z' => 0,
                _ => todo!(),
            },
            'B' => match second {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => todo!(),
            },
            'C' => match second {
                'X' => 6,
                'Y' => 0,
                'Z' => 3,
                _ => todo!(),
            },
            _ => todo!(),
        };

        let shape = match second {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => todo!(),
        };

        total_score += outcome + shape;
    }
    total_score
}

fn part2(input: &str) -> i32 {
    let input: Vec<&str> = input.split("\n").collect();
    let mut total_score = 0;

    for i in input {
        if i.len() < 3 {
            continue;
        }
        let mut chars = i.chars();
        let first = chars.next().unwrap();
        chars.next();
        let second = chars.next().unwrap();

        let outcome = match first {
            'A' => match second {
                'X' => 0 + 3,
                'Y' => 3 + 1,
                'Z' => 6 + 2,
                _ => todo!(),
            },
            'B' => match second {
                'X' => 0 + 1,
                'Y' => 3 + 2,
                'Z' => 6 + 3,
                _ => todo!(),
            },
            'C' => match second {
                'X' => 0 + 2,
                'Y' => 3 + 3,
                'Z' => 6 + 1,
                _ => todo!(),
            },
            _ => todo!(),
        };

        total_score += outcome;
    }
    total_score
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use pretty_assertions::assert_eq;

    use crate::{part1, part2};

    #[test]
    fn day2test1() {
        let input = "A Y\n\
        B X\n\
        C Z\n";
        let output = part1(&input);
        assert_eq!(output, 15)
    }

    #[test]
    fn day2test2() {
        let input = "A Y\n\
        B X\n\
        C Z\n";
        let output = part2(&input);
        assert_eq!(output, 12)
    }

    #[bench]
    fn day2speedtest(b: &mut Bencher) {
        let path = "day2input.txt";
        let input = fs::read_to_string(path);
        let input = match input {
            Ok(s) => s,
            Err(_) => panic!("input file not found"),
        };

        b.iter(|| {
            part1(&input[..]);
            part2(&input[..]);
        });
    }
}
