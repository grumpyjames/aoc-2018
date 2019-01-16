use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("src/day1input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut result = 0;
    let mut set = HashSet::new();
    let input =
        file.lines()
            .map(| l | l.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
    let repeat = input.iter().cycle();

    for delta in repeat {
        result += delta;
        let new = set.insert(result);
        if !new
        {
            println!("{}", result);
            break;
        }
    }

}
