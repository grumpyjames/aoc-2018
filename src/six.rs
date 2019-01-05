use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Claim {
    claimant: (u32, u32),
    distance: u32
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Unclaimed,
    Highlander(Claim),
    Twinned(Claim, Claim)
}

fn manhattan_distance(from: (u32, u32), to: (u32, u32)) -> u32 {
    (from.1 as i32 - to.1 as i32).abs() as u32 + (from.0 as i32 - to.0 as i32).abs() as u32
}

fn main() {
    let file = File::open("src/day6input.txt").unwrap();
    let points : Vec<(u32, u32)> =
        BufReader::new(file)
            .lines()
            .map(|line| {
        let split : Vec<u32> =
            line.unwrap().split(", ").map(|a| {
                a.parse::<u32>().unwrap()
            }).collect();
        (split[0], split[1])
    }).collect();

    let mut max_x = 0;
    let mut max_y = 0;
    points.iter().for_each(|p| {
        if p.0 > max_x {
            max_x = p.0
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    });
    max_x = max_x + 1;
    max_y = max_y + 1;
    println!("{} by {}", max_x, max_y);

    let mut cell_rows: Vec<Vec<Cell>> = Vec::with_capacity(max_y as usize);
    let mut total_rows: Vec<Vec<u32>> = Vec::with_capacity(max_y as usize);
    for _i in 0..max_y {
        let mut row = Vec::with_capacity(max_x as usize);
        let mut tot_row = Vec::with_capacity(max_x as usize);
        for _j in 0..max_x {
            row.push(Cell::Unclaimed);
            tot_row.push(0);
        }
        cell_rows.push(row);
        total_rows.push(tot_row);
    }

    points.iter().for_each(|p| {
        for i in 0..max_y {
            for j in 0..max_x {
                let current_claim = cell_rows[i as usize][j as usize];
                let distance = manhattan_distance(*p, (j, i));
                total_rows[i as usize][j as usize] += distance;
                let new_claim = match current_claim {
                    Cell::Unclaimed => {
                        Cell::Highlander(Claim {
                            claimant: *p,
                            distance: distance
                        })
                    },
                    Cell::Highlander(c) => {
                        if distance < c.distance {
                            Cell::Highlander(Claim {
                                claimant: *p,
                                distance: distance
                            })
                        } else if distance == c.distance {
                            Cell::Twinned(c.clone(), Claim {
                                claimant: *p,
                                distance: distance
                            })
                        } else {
                            Cell::Highlander(c)
                        }
                    },
                    Cell::Twinned(c1, c2) => {
                        if distance < c1.distance {
                            Cell::Highlander(Claim {
                                claimant: *p,
                                distance: distance
                            })
                        } else {
                            Cell::Twinned(c1, c2)
                        }
                    }
                };
                cell_rows[i as usize][j as usize] = new_claim;
            }
        }
    });

    let mut results = HashMap::new();
    for i in 0..max_y {
        for j in 0..max_x {
            let current_claim = cell_rows[i as usize][j as usize];
            match current_claim {
                Cell::Highlander(c) => {
                    let x = results
                        .entry(c.claimant)
                        .or_insert(0);
                    *x += 1;
                },
                _ => {}
            }
        }
    }

    for i in [0, max_y - 1].iter() {
        for j in 0..max_x {
            let current_claim = cell_rows[*i as usize][j as usize];
            match current_claim {
                Cell::Highlander(c) => {
                    let option = results.remove(&c.claimant);
                    match option {
                        Some(_a) => println!("removed {},{}, because {},{}",
                                            c.claimant.0, c.claimant.1, j, i),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    for j in [0, max_x - 1].iter() {
        for i in 0..max_y {
            let current_claim = cell_rows[i as usize][*j as usize];
            match current_claim {
                Cell::Highlander(c) => {
                    let option = results.remove(&c.claimant);
                    match option {
                        Some(_a) => println!("removed {},{}, because {},{}",
                                            c.claimant.0, c.claimant.1, j, i),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    let mut max_area = 0;
    results.iter()
        .for_each(|(k, v)| {
            if *v > max_area {
                max_area = *v;
                println!("({},{}) {}", k.0, k.1, v);
            }
        });

    let mut region_area = 0;
    for i in 0..max_y {
        for j in 0..max_x {
            let cell_total = total_rows[i as usize][j as usize];
            if cell_total < 10000 {
                region_area += 1;
            }
        }
    }

    println!("{}", region_area);
}
