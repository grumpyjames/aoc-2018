extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;
use std::cmp::max;

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pixel {
    x: u32,
    y: u32
}

fn parse_rectangle(line: &String) -> Rectangle
{
    let re = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    let option = re.captures(line);

    match option
    {
        None => { panic!("no rectangle available from {}", line) },
        Some(x) => {
            Rectangle {
                id : x[1].parse().unwrap(),
                left: x[2].parse().unwrap(),
                top: x[3].parse().unwrap(),
                width: x[4].parse().unwrap(),
                height: x[5].parse().unwrap(),
            }
        },
    }
}

fn area(r: &Rectangle) -> u32
{
    r.width * r.height
}

fn pixels(r: &Rectangle) -> Vec<Pixel>
{
    let mut result = Vec::with_capacity(area(r) as usize);
    for x in r.left..(r.left + r.width) {
        for y in r.top..(r.top + r.height) {
            result.push(Pixel { x, y })
        }
    }

    result
}

fn main() {
    let f = File::open("src/day3input.txt").unwrap();
    let file = BufReader::new(&f);

    let rs: Vec<Rectangle> = file.lines().map(|l| parse_rectangle(&l.unwrap())).collect();

    rs.iter().for_each(|r| println!("{:?}", r));

    let max_bounds = rs.iter().fold((0, 0), |acc, item| {
       (max(item.left + item.width, acc.0), max(item.top + item.height, acc.1))
    });

    println!("{:?}", max_bounds);
    let mut tapestry : Vec<Vec<u8>> = Vec::with_capacity(max_bounds.1 as usize);
    for _y in 0..max_bounds.1
    {
        let mut row: Vec<u8> = Vec::with_capacity(max_bounds.0 as usize);
        for _x in 0..max_bounds.0
        {
            row.push(0);
        };
        tapestry.push(row);
    };

    for r in rs.iter()
    {
        pixels(r).iter().for_each(|p| {
           tapestry[p.y as usize][p.x as usize] += 1;
        });
    };

    for r in rs.iter()
    {
        let mut overlap_count : u32 = 0;
        pixels(r).iter().for_each(|p| {
            overlap_count += tapestry[p.y as usize][p.x as usize] as u32;
        });

        if overlap_count == area(r)
        {
            println!("{:?}", r);
        }
    };

    let mut count = 0;
    for y in 0..max_bounds.1
    {
        for x in 0..max_bounds.0
        {
            let val = tapestry[y as usize][x as usize];
            print!("{}", val);
            if val > 1
            {
                count += 1;
            };
        };
        print!("\n");
    };
    println!("{:?}", count);
}
