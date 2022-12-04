#![feature(test)]

extern crate test;
use std::fs;

fn main() {
    let path = "./day1input.txt";
    let contents = fs::read_to_string(path)
        .expect("couldn't find file");
    let sum = part1(&contents);
    println!("{}", sum);

    let sum = part2(&contents);
    println!("{}", sum);
}

fn part1(contents: &String) -> i32 {
    
    let mut sum = 0;
    let mut greatest = 0;
    for i in contents.split('\n') {
        if let Ok(n) = i.parse::<i32>() {
            sum += n;
        } else {
            if sum > greatest {
                greatest = sum;
            }
            sum = 0;
        }
    }
    if sum > greatest {
        greatest = sum;
    }
    greatest
}

fn part2(contents: &String) -> i32 {
    let mut queue: Vec<i32> = Vec::new();

    let mut sum = 0;
    for i in contents.split("\n") {
        if let Ok(n) = i.parse::<i32>() {
            sum += n;
        } else {
            if queue.len() == 0 {
                queue.push(sum);
            }
                else {
                for i in 0..queue.len() {
                    if let Some(n) = queue.get(i) {
                        if sum > *n {
                            queue.insert(i, sum);
                            break;
                        }
                    }
                }
            }
            if queue.len() > 3 {
                queue.pop();
            }
            sum = 0;
        }
    }
    return queue.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::fs;
    use test::Bencher;

    use pretty_assertions::assert_eq;

    use crate::part1;
    use crate::part2;

    #[test]

    fn test1() {
        let input = 
        "1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000\n";
        
        let input = String::from_str(input)
            .expect("couldn't convert String");
        let output = part1(&input);
        assert_eq!(output, 24000)
    }

    #[test]
    fn test2() {
        let input = 
        "1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000\n";

        if let Ok(input) = String::from_str(input) {
            let output = part2(&input);
            assert_eq!(45000, output);
        } else {
            panic!("Unable to convert to string");
        }
    }
     
    #[bench]
    fn speedtest(b: &mut Bencher) {
        let path = "./day1input.txt";
        let contents = fs::read_to_string(path)
            .expect("couldn't find file");
        
        b.iter(|| {
            part1(&contents);
            part2(&contents);
        });
    }
}