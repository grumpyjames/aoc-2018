extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use regex::Regex;
use std::collections::HashSet;

fn main() {
    let file = File::open("src/day7input.txt").unwrap();
    let re =
        Regex::new(r"Step (.) must be finished before step (.) can begin.")
            .unwrap();
    // a -> b means a depends on b
    let mut deps = HashMap::new();
    // a -> b means b depends on a
    let mut reverse_deps = HashMap::new();
    BufReader::new(file)
            .lines()
            .for_each(|line| {
        for cap in re.captures_iter(&line.unwrap()) {
            deps.entry(cap[2].to_string())
                .or_insert(HashSet::new())
                .insert(cap[1].to_string());
            reverse_deps.entry(cap[1].to_string())
                .or_insert(Vec::new())
                .push(cap[2].to_string());
            deps.entry(cap[1].to_string()).or_insert(HashSet::new());
        }
    });

    while !deps.is_empty() {
        let mut done : Vec<String> = Vec::new();
        deps.iter().for_each(|(k, v)| {
            if v.is_empty() {
                done.push(k.clone());
            }
        });
        done.sort();

        if !done.is_empty() {
            let k = done[0].clone();
            deps.remove(&k);
            print!("{}", k);

            let option = reverse_deps.get(&k);
            match option {
                Some(vec) => {
                    vec.iter().for_each(|k2| {
                        let option_two = deps.get_mut(k2);
                        match option_two {
                            Some(set) => {
                                set.remove(&k);
                            },
                            _ => {}
                        }
                    })
                },
                _ => {}
            }
        }
    }
}
