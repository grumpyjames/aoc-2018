use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let f = File::open("src/day1input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut result = 0;

    for line in file.lines() {
        let delta = line.unwrap().parse::<i32>().unwrap();
        result += delta;
    }

    println!("{}", result);
}
