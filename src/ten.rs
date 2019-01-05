extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug)]
struct Star
{
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32
}

impl Star
{
    fn tick(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
    }
}

fn spread(stars: & Vec<Star>) -> ((i32, i32), (i32, i32)) {
    let mut min_x = 65536;
    let mut max_x = -65536;
    let mut min_y = 65536;
    let mut max_y = -65536;
    stars.iter().for_each(|s| {
        min_x = min_x.min(s.x);
        max_x = max_x.max(s.x);
        min_y = min_y.min(s.y);
        max_y = max_y.max(s.y);
    });

    ((min_x, min_y), (max_x, max_y))
}

fn main() {
    let file = File::open("src/day10input.txt").unwrap();
    // position=< 5,  9> velocity=< 1, -2>
    let re =
        Regex::new(r"position=<\s?(-?[0-9]+),\s+(-?[0-9]+)> velocity=<\s?(-?[0-9]+),\s+(-?[0-9]+)>")
            .unwrap();
    let mut stars = Vec::new();

    BufReader::new(file)
        .lines()
        .for_each(|line| {
            for cap in re.captures_iter(&line.unwrap()) {
                stars.push(Star {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                    v_x: cap[3].parse().unwrap(),
                    v_y: cap[4].parse().unwrap(),
                });
            }
        });

    // let view_dimensions = (30, 30);
    //  let origin = (10, 10);

    for h in 0..50000 {
        let (min, max) = spread(&stars);
        let spread_factor = ((max.0 - min.0) as i64) * ((max.1 - min.0) as i64);
        //println!("{}", spread_factor);
        if spread_factor < 4000 {
            println!("{}, {}, {}, {}", min.0, min.1, max.0, max.1);
            println!("Stars are aligning at second {}", h);
            for j in min.1..=max.1 {
                for i in min.0..=max.0 {
                    if stars.iter()
                        .any(|star| star.x == i && star.y == j)
                    {
                        print!("*");
                    }
                    else
                    {
                        print!(".");
                    }
                }
                println!("")
            }
        }

        stars.iter_mut().for_each(|s| {
            s.tick()
        })
    }

//    println!("{:?}", stars);
}