use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("src/day2input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut threes = 0;
    let mut twos = 0;

    for line in file.lines() {
        let ffs = line.unwrap();
        let map = process_line(&ffs);

        println!("{}", ffs);
        let mut two_seen = false;
        let mut three_seen = false;
        for (k, v) in map {
            println!("({}, {})", k, v);
            if v == 3 && !three_seen
            {
                threes += 1;
                three_seen = true;
            }
            if v == 2 && !two_seen
            {
                twos += 1;
                two_seen = true;
            }
        }
    }

    println!("{}", twos * threes);
}

fn process_line(line: &String) -> HashMap<u8, i32, RandomState> {
    let mut map = HashMap::new();
    for c in line.as_bytes() {
        let existing = map.get(c).map(|v| *v).unwrap_or(0);
        map.insert(c.clone(), existing + 1);
    }
    map
}
