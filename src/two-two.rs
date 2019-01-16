use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Thing {
    Start,
    Candidate(String),
    Solution(String, String)
}

fn main() {
    let f = File::open("src/day2input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut x: Vec<String> = file.lines().map(|l| l.unwrap()).collect();

    x.sort_by(|a, b| a.cmp(b));

    let result = x.iter().fold(Thing::Start, |acc, item| {
        match acc
        {
            Thing::Start => Thing::Candidate(item.clone()),
            Thing::Candidate(x) => {
                let mut diff = 0;
                for (c1, c2) in x.as_bytes().iter().zip(item.as_bytes().iter()) {
                    if c1 != c2 {
                        diff += 1
                    }
                }
                if diff == 1
                {
                    Thing::Solution(x, item.clone())
                }
                else
                {
                    Thing::Candidate(item.clone())
                }
            },
            Thing::Solution(x, y) => Thing::Solution(x, y)
        }
    });

    println!("{:?}", result);
}
