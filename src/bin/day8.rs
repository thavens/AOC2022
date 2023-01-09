use std::fs;
fn main() {
    let path = "day8input.txt";
    let contents = fs::read_to_string(path).expect("File should exist");
    let arr = str2arr(&contents[..]);
    let output = part1(&arr);
    println!("{}", output);
    let output = part2(&arr);
    println!("{}", output);
}

fn part1(input: &Vec<Vec<i8>>) -> i32 {
    let mut large: Vec<Vec<bool>> = Vec::new();
    for _i in 0..input.len() {
        let mut med: Vec<bool> = Vec::new();
        for _j in 0..input[0].len() {
            med.push(false);
        }
        large.push(med);
    }
    
    for i in 0..input.len() {
        let mut max = -1;
        for j in 0..input[0].len() {
            if input[i][j] > max {
                large[i][j] = true;
                max = input[i][j];
            }
        }

        max = -1;
        for j in (0..input[0].len()).rev() {
            if input[i][j] > max {
                large[i][j] = true;
                max = input[i][j];
            }
        }
    }

    for j in 0..input[0].len() {
        let mut max = -1;
        for i in 0..input.len() {
            if input[i][j] > max {
                large[i][j] = true;
                max = input[i][j];
            }
        }

        max = -1;
        for i in (0..input.len()).rev() {
            if input[i][j] > max {
                large[i][j] = true;
                max = input[i][j];
            }
        }
    }

    let mut sum = 0;
    large.iter().for_each(|line| {
        line.iter().for_each(|elem| {
            if *elem {print!("#");} else {print!(" ");}
            sum += if *elem {1} else {0}
        });
        println!();
    });
    sum
}

fn part2(input: &Vec<Vec<i8>>) -> i32 {
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let index = vis_index(&input, i, j);
            if index > max {
                max = index;
            }
        }
    }
    max
}

fn vis_index(input: &Vec<Vec<i8>>, i: usize, j: usize) -> i32 {
    let height = input[i][j];
    let mut lsize = 0;
    for j in (0..j).rev() {
        if input[i][j] < height {
            lsize += 1;
        }
        else if input[i][j] == height {
            lsize += 1;
            break;
        }
    }

    let mut rsize = 0;
    for j in (j+1)..input[0].len() {
        if input[i][j] < height {
            rsize += 1;
        }
        else if input[i][j] >= height {
            rsize += 1;
            break;
        }
    }

    let mut usize = 0;
    for i in (0..i).rev() {
        if input[i][j] < height {
            usize += 1;
        }
        else if input[i][j] == height {
            usize += 1;
            break;
        }
    }

    let mut dsize = 0;
    for i in (i+1)..input.len() {
        if input[i][j] < height {
            dsize += 1;
        }
        else if input[i][j] == height {
            dsize += 1;
            break;
        }
    }

    lsize * rsize * usize * dsize
}

fn str2arr(input: &str) -> Vec<Vec<i8>> {
    let mut large: Vec<Vec<i8>> = Vec::new();
    input.lines().for_each(|line| {
        let mut med: Vec<i8> = Vec::new();
        line.chars().for_each(|letter| {
            med.push(letter.to_digit(10).unwrap() as i8);
        });
        large.push(med);
    });
    large
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{part1, part2, str2arr, vis_index};

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn day8test1() {
        let array = str2arr(INPUT);
        let output = part1(&array);
        assert_eq!(output, 21);
    }

    #[test]
    fn day8test2() {
        let array = str2arr(INPUT);
        let output = part2(&array);
        assert_eq!(output, 8)
    }

    #[test]
    fn day8test_vis_index() {
        let array = str2arr(INPUT);
        let output1 = vis_index(&array, 1, 2);
        let output2 = vis_index(&array, 3, 2);
        assert_eq!(output1, 4);
        assert_eq!(output2, 8);
    }
}