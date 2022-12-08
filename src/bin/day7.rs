use std::fs;

fn main() {

}

struct Folder {
    name: String,
    size: i32
}

impl Folder {
    fn new(name: String, size: i32) -> Folder {
        Folder {name, size}
    }
}

fn part1(contents: &str) {
    for i in contents.lines() {
        let chars = i.as_bytes();
        if i
    }
}