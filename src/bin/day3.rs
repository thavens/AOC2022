#![feature(test)]
extern crate test;
use std::collections::{HashSet, HashMap};
use std::fs;

fn main() {
    let path = "day3input.txt";
    let contents = fs::read_to_string(path).expect("Should've found file here");
    let result = part1(&contents[..]);
    println!("part1: {}", result);
    let result = part2(&contents[..]);
    println!("part2: {}", result)
}

fn part1(contents: &str) -> u32 {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut score = 0;
    for i in contents {
        let mut freq: HashSet<char> = HashSet::new();
        let mut dups: HashSet<char> = HashSet::new();
        for j in (&(*i)[..i.len() / 2]).chars() {
            freq.insert(j);
        }
        for j in (&(*i)[i.len() / 2..]).chars() {
            if freq.contains(&j) && !dups.contains(&j) {
                dups.insert(j);
                score += char2val(j);
            }
        }
    }
    score
}

fn part2(contents: &str) -> u32 {
    let contents: Vec<&str> = contents.split("\n").collect();
    let mut score = 0;
    for i in (0..contents.len()).step_by(3) {
        let mut count: HashMap<char, u32> = HashMap::new();
        for j in contents[i].chars() {
            count.insert(j, 1);
        }
        for j in contents[i+1].chars() {
            let counter = count.entry(j).or_insert(0);
            if *counter == 1 {
                *counter += 1;
            }
        }
        for j in contents[i+2].chars() {
            let counter = count.entry(j).or_insert(0);
            if *counter == 2 {
                *counter += 1;
            }
        }
        for j in count.keys() {
            if *count.get(j).unwrap() == 3 {
                score += char2val(*j);
                break;
            }
        }
    }
    score
}

fn char2val(j: char) -> u32{
    let value = j as u32;
    let mut score = 0;
    if j.is_ascii_uppercase() {
        score += value - 65 + 27;
    } else {
        score += value - 96;
    }
    score
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use pretty_assertions::assert_eq;

    use crate::{part1, part2};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day3test1() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn day3test2() {
        let result = part2(INPUT);
        assert_eq!(result, 70);
    }

    #[bench]
    fn day3speedtest(b: &mut Bencher) {
        let path = "day3input.txt";
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
