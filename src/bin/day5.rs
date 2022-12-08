#![feature(test)]
extern crate test;
use std::fs;
use std::str;
use regex::Regex;

fn main() {
    let path = "day5input.txt";
    let input = fs::read_to_string(path).expect("Should find file.");
    let output = part1(&input[..]);
    println!("part1: {}", output);
    let output = part2(&input[..]);
    println!("Part2: {}", output);
}

fn part1(contents: &str) -> String {
    let contents: Vec<&str> = contents.split("\n").collect();
    let width = (contents[0].len() - 3) / 4 + 1;
    let height = find_height(&contents);
    let mut stacks = buildstacks(width, &contents[0..height]);
    let contents = &contents[height + 2..contents.len()];
    let finder = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for &i in contents {
        let mut caps = finder.captures(i).unwrap();
        let times: usize = caps[1].parse().unwrap();
        let from: usize = caps[2].parse().unwrap();
        let to: usize = caps[3].parse().unwrap();
        for j in 0..times {
            let temp = stacks[from-1].pop().unwrap();
            stacks[to-1].push(temp);
        }
    }
    let mut tops: Vec<char> = Vec::new();
    for i in stacks {
        tops.push(i[i.len() - 1]);
    }
    let string: String = tops.into_iter().collect();
    string
}

fn buildstacks(width: usize, contents: &[&str]) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(width);
    for i in 0..width {
        stacks.push(Vec::new());
    }
    for &i in contents.iter().rev() {
        for (stack, val) in i.as_bytes().iter().skip(1).step_by(4).enumerate() {
            if *val != ' ' as u8 {
                stacks[stack].push(*val as char);
            }
        }
    }
    stacks
}

fn find_height(contents: &Vec<&str>) -> usize {
    for (i, val) in (*contents).iter().enumerate() {
        if val.as_bytes()[1].is_ascii_digit() {
            return i;
        }
    }
    0
}

fn part2(contents: &str) -> String {
    let contents: Vec<&str> = contents.split("\n").collect();
    let width = (contents[0].len() - 3) / 4 + 1;
    let height = find_height(&contents);
    let mut stacks = buildstacks(width, &contents[0..height]);
    let contents = &contents[height + 2..contents.len()];
    let finder = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut temp: Vec<char> = Vec::new();
    for &i in contents {
        let mut caps = finder.captures(i).unwrap();
        let times: usize = caps[1].parse().unwrap();
        let from: usize = caps[2].parse().unwrap();
        let to: usize = caps[3].parse().unwrap();
        for j in 0..times {
            temp.push(stacks[from-1].pop().unwrap());
        }
        for j in 0..times {
            stacks[to - 1].push(temp.pop().unwrap());
        }
    }
    let mut tops: Vec<char> = Vec::new();
    for i in stacks {
        tops.push(i[i.len() - 1]);
    }
    let string: String = tops.into_iter().collect();
    string
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use pretty_assertions::assert_eq;

    use crate::{part1, part2};

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn day5test1() {
        let output = part1(INPUT);
        assert_eq!(output, "CMZ");
    }

    #[test]
    fn day5test2() {
        let output = part1(INPUT);
        assert_eq!(output, "MCD");
    }

    #[bench]
    fn day5bench(b: &mut Bencher) {
        let path = "day5input.txt";
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