#![feature(test)]
extern crate test;
use std::fs;
fn main() {
    let path = "day4input.txt";
    let input = fs::read_to_string(path).expect("File should exist");
    let output = part1(&input[..]);
    println!("part1: {}", output);
    let output = part2(&input[..]);
    println!("part2: {}", output);
}

fn part1(contents: &str) -> i32 {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut overlaps = 0;
    for i in contents{
        let pairs:Vec<&str> = i.split(|input: char| -> bool {input == ',' || input == '-'}).collect();
        let x1: i32 = pairs[0].parse().unwrap();
        let x2: i32 = pairs[1].parse().unwrap();
        let y1: i32 = pairs[2].parse().unwrap();
        let y2: i32 = pairs[3].parse().unwrap();
        if x1 >= y1 && x2 <= y2 || x1 <= y1 && x2 >= y2 {
            overlaps += 1;
        }
    }
    overlaps
}

fn part2(contents: &str) -> i32 {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut overlaps = 0;
    for i in contents{
        let pairs:Vec<&str> = i.split(|input: char| -> bool {input == ',' || input == '-'}).collect();
        let x1: i32 = pairs[0].parse().unwrap();
        let x2: i32 = pairs[1].parse().unwrap();
        let y1: i32 = pairs[2].parse().unwrap();
        let y2: i32 = pairs[3].parse().unwrap();

        if x1 >= y1 && x1 <= y2 || x2 >= y1 && x2 <= y2 || y1 >= x1 && y1 <= x2 {
            overlaps += 1;
        }
    }
    overlaps
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use pretty_assertions::assert_eq;

    use crate::{part1, part2};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn day4test1() {
        let output = part1(INPUT);
        assert_eq!(output, 2);
    }

    #[test]
    fn day4test2() {
        let output = part2(INPUT);
        assert_eq!(output, 4);
    }

    #[bench]
    fn day4bench(b: &mut Bencher) {
        let path = "day4input.txt";
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