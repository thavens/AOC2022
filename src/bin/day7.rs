use std::{collections::HashMap, fs};

fn main() {
    let path = "day7input.txt";
    let input = fs::read_to_string(path).expect("Should fine file");
    let output = part1(&input[..]);
    println!("Part 1: {}", output);
    let output = part2(&input[..]);
    println!("Part 2: {}", output);
}

fn part1(contents: &str) -> u64 {
    let mut cwd: Vec<&str> = Vec::new();
    let mut dsize: HashMap<String, u64> = HashMap::new();
    for i in contents.lines() {
        let tokens: Vec<&str> = i.split(" ").collect();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    let child = cwd.pop().expect("Should be here.");
                    let &child_size = dsize
                        .get(&make_path(&cwd, child))
                        .expect("There should be a size value");
                    dsize
                        .entry(make_path(&cwd, ""))
                        .and_modify(|size| *size += child_size);
                } else {
                    cwd.push(tokens[2]);
                    dsize.entry(make_path(&cwd, "")).or_insert(0);
                }
            } else if tokens[1] == "ls" {
            }
        } else if tokens[0] == "dir" {
        } else {
            dsize.entry(make_path(&cwd, "")).and_modify(|size| {
                *size += tokens[0].parse::<u64>().unwrap();
            });
        }
    }

    for _i in 0..cwd.len() - 1 {
        let child = cwd.pop().expect("there should be a child in here");
        let &child_size = dsize
            .get(&make_path(&cwd, child))
            .expect("There should be a size value");
        dsize
            .entry(make_path(&cwd, ""))
            .and_modify(|size| *size += child_size);
    }
    dsize.values().filter(|size| **size <= 100000).sum()
}

fn make_path(path: &Vec<&str>, end: &str) -> String {
    let mut full = path.join("/");
    if !end.is_empty() {
        full.push('/');
        full.push_str(end);
    }
    full
}

fn part2(contents: &str) -> u64 {
    let mut cwd: Vec<&str> = Vec::new();
    let mut dsize: HashMap<String, u64> = HashMap::new();
    for i in contents.lines() {
        let tokens: Vec<&str> = i.split(" ").collect();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    let child = cwd.pop().expect("Should be here.");
                    let &child_size = dsize
                        .get(&make_path(&cwd, child))
                        .expect("There should be a size value");
                    dsize
                        .entry(make_path(&cwd, ""))
                        .and_modify(|size| *size += child_size);
                } else {
                    cwd.push(tokens[2]);
                    dsize.entry(make_path(&cwd, "")).or_insert(0);
                }
            } else if tokens[1] == "ls" {
            }
        } else if tokens[0] == "dir" {
        } else {
            dsize.entry(make_path(&cwd, "")).and_modify(|size| {
                *size += tokens[0].parse::<u64>().unwrap();
            });
        }
    }

    for _i in 0..cwd.len() - 1 {
        let child = cwd.pop().expect("there should be a child in here");
        let &child_size = dsize
            .get(&make_path(&cwd, child))
            .expect("There should be a size value");
        dsize
            .entry(make_path(&cwd, ""))
            .and_modify(|size| *size += child_size);
    }

    let total_size: u64 = *dsize.get(&String::from("/")).unwrap();
    let size_needed = total_size + 30000000 - 70000000;
    let mut min = u64::MAX;
    let mut best_key = String::new();
    dsize.drain().for_each(|(key, value)| {
        if value > size_needed && value < min {
            best_key = key;
            min = value;
        }
    });
    min
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{part1, part2};
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn day7test1() {
        let output = part1(INPUT);
        assert_eq!(output, 95437);
    }

    #[test]
    fn day7test2() {
        let output = part2(INPUT);
        assert_eq!(output, 24933642);
    }
}
