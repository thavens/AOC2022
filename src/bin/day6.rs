use std::fs;
use std::collections::VecDeque;
fn main() {
    let path = "day6input.txt";
    let input = fs::read_to_string(path).expect("Should find the file");
    let output = part1(&input[..]);
    println!("{}", output);
    let output = part2(&input[..]);
    println!("{}", output);
}

fn part1(contents: &str) -> i32{
    let mut arr = [0; 26];
    let mut queue: VecDeque<char> = VecDeque::new();
    for (i, val) in contents.chars().enumerate() {
        arr[val as usize - 'a' as usize] += 1;
        queue.push_back(val);
        if queue.len() == 5 {
            arr[queue.pop_front().unwrap() as usize - 'a' as usize] -= 1;
        }
        if arr.iter().all(|x: &i32| {*x < 2}) && arr.iter().sum::<i32>() == 4 {
            return i as i32 + 1;
        }
    }
    -1
}

fn part2(contents: &str) -> i32{
    let mut arr = [0; 26];
    let mut queue: VecDeque<char> = VecDeque::new();
    for (i, val) in contents.chars().enumerate() {
        arr[val as usize - 'a' as usize] += 1;
        queue.push_back(val);
        if queue.len() == 15 {
            arr[queue.pop_front().unwrap() as usize - 'a' as usize] -= 1;
        }
        if arr.iter().all(|x: &i32| {*x < 2}) && arr.iter().sum::<i32>() == 14 {
            return i as i32 + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{part1, part2};

    const INPUT: [&str; 5] = ["mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];
    
    const OUTPUT1: [i32; 5] = [7, 5, 6, 10, 11];
    const OUTPUT2: [i32; 5] = [19, 23, 23, 29, 26];

    #[test]
    fn day6test1() {
        for i in 0..INPUT.len() {
            assert_eq!(part1(INPUT[i]), OUTPUT1[i]);
        }
    }

    #[test]
    fn day6test2() {
        for i in 0..INPUT.len() {
            assert_eq!(part2(INPUT[i]), OUTPUT2[i]);
        }
    }
}