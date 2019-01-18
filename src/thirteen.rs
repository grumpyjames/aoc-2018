extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Debug, Copy, Clone)]
enum Track {
    Vertical,
    Across,
    ForwardDiagonal,
    BackwardDiagonal,
    Intersection
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Cart {
    cart_id: usize,
    x: usize,
    y: usize,
    dir: Direction,
    intersections: usize
}

impl Cart {
    fn to_string(&self, tracks: &Vec<Vec<Option<Track>>>) -> String {
        format!(
            "{}: ({}, {}), {}, {}, {:?}",
            self.cart_id,
            self.x,
            self.y,
            to_string(tracks[self.y][self.x].unwrap()),
            self.intersections,
            self.dir)
    }

    fn tick(&mut self, tracks: &Vec<Vec<Option<Track>>>) {
        let t = tracks[self.y][self.x].expect("there should be track here :-(");

        match (t, self.dir) {
            (Track::Intersection, Direction::Up) => {
                if self.intersections == 0 {
                    self.dir = Direction::Left;
                    self.x -= 1;
                    self.intersections = 1;
                } else if self.intersections == 1 {
                    self.y -= 1;
                    self.intersections = 2
                } else if self.intersections == 2 {
                    self.dir = Direction::Right;
                    self.x += 1;
                    self.intersections = 0
                }
            }
            (Track::Intersection, Direction::Down) => {
                if self.intersections == 0 {
                    self.dir = Direction::Right;
                    self.x += 1;
                    self.intersections = 1;
                } else if self.intersections == 1 {
                    self.y += 1;
                    self.intersections = 2
                } else if self.intersections == 2 {
                    self.dir = Direction::Left;
                    self.x -= 1;
                    self.intersections = 0
                }
            }
            (Track::Intersection, Direction::Left) => {
                if self.intersections == 0 {
                    self.dir = Direction::Down;
                    self.y += 1;
                    self.intersections = 1;
                } else if self.intersections == 1 {
                    self.x -= 1;
                    self.intersections = 2
                } else if self.intersections == 2 {
                    self.dir = Direction::Up;
                    self.y -= 1;
                    self.intersections = 0
                }
            }
            (Track::Intersection, Direction::Right) => {
                if self.intersections == 0 {
                    self.dir = Direction::Up;
                    self.y -= 1;
                    self.intersections = 1;
                } else if self.intersections == 1 {
                    self.x += 1;
                    self.intersections = 2
                } else if self.intersections == 2 {
                    self.dir = Direction::Down;
                    self.y += 1;
                    self.intersections = 0
                }
            }
            (Track::Across, Direction::Left) => {
                self.x -= 1;
            }
            (Track::Across, Direction::Right) => {
                self.x += 1;
            }
            (Track::Vertical, Direction::Up) => {
                self.y -= 1;
            }
            (Track::Vertical, Direction::Down) => {
                self.y += 1;
            }
            (Track::ForwardDiagonal, Direction::Up) => {
                self.dir = Direction::Right;
                self.x += 1;
            }
            (Track::ForwardDiagonal, Direction::Down) => {
                self.dir = Direction::Left;
                self.x -= 1;
            }
            (Track::ForwardDiagonal, Direction::Left) => {
                self.dir = Direction::Down;
                self.y += 1;
            }
            (Track::ForwardDiagonal, Direction::Right) => {
                self.dir = Direction::Up;
                self.y -= 1;
            }
            (Track::BackwardDiagonal, Direction::Up) => {
                self.dir = Direction::Left;
                self.x -= 1;
            }
            (Track::BackwardDiagonal, Direction::Down) => {
                self.dir = Direction::Right;
                self.x += 1;
            }
            (Track::BackwardDiagonal, Direction::Left) => {
                self.dir = Direction::Up;
                self.y -= 1;
            }
            (Track::BackwardDiagonal, Direction::Right) => {
                self.dir = Direction::Down;
                self.y += 1;
            }

            // Something terrible has happened - panic?
            (Track::Across, Direction::Up) => { panic!("Impossible!")}
            (Track::Across, Direction::Down) => { panic!("Impossible!") }
            (Track::Vertical, Direction::Left) => { panic!("Impossible!") }
            (Track::Vertical, Direction::Right) => { panic!("Impossible!") }
        }
    }
}

fn main() {
    let file = File::open("src/day13input.txt").unwrap();

    let mut track_rows = Vec::new();
    let mut carts = Vec::new();
    let mut y = 0;
    let mut cart_id = 0;

    BufReader::new(file)
        .lines()
        .for_each(|line| {
            let mut row = Vec::new();
            let mut x = 0;
            line.unwrap()
                .as_bytes()
                .iter()
                .for_each(|b| {
                    row.push(match *b as char {
                        '|' => Some(Track::Vertical),
                        '-' => Some(Track::Across),
                        '/' => Some(Track::ForwardDiagonal),
                        '\\' => Some(Track::BackwardDiagonal),
                        '+' => Some(Track::Intersection),
                        '>' => Some(Track::Across),
                        '<' => Some(Track::Across),
                        '^' => Some(Track::Vertical),
                        'v' => Some(Track::Vertical),
                        _ => None
                    });

                    match *b as char {
                        '^' => {
                            carts.push(Cart { cart_id, x, y, dir: Direction::Up, intersections: 0});
                            cart_id += 1;
                        },
                        'v' => {
                            carts.push(Cart { cart_id, x, y, dir: Direction::Down, intersections: 0});
                            cart_id += 1;
                        },
                        '>' => {
                            carts.push(Cart { cart_id, x, y, dir: Direction::Right, intersections: 0});
                            cart_id += 1;
                        },
                        '<' => {
                            carts.push(Cart { cart_id, x, y, dir: Direction::Left, intersections: 0});
                            cart_id += 1;
                        },
                        _ => {}
                    }

                    x += 1;
                });
            track_rows.push(row);
            y += 1;
        });

    let mut removed : HashSet<usize> = HashSet::new();

    while carts.len() - removed.len() > 1 {
        tick(&track_rows, &mut carts, &mut removed);
    }
    //tick(&track_rows, &mut carts, &mut removed);
    print_world(&track_rows, &carts);

    for c in carts {
        if !removed.contains(&c.cart_id) {
            println!("{:?}", c.to_string(&track_rows));
        }
    }
}

fn tick(
    track_rows: &Vec<Vec<Option<Track>>>,
    carts: &mut Vec<Cart>,
    removed: &mut HashSet<usize>) {

    let mut positions = HashMap::new();
    carts.iter().for_each(|c| {
        if !removed.contains(&c.cart_id) {
            positions
                .entry((c.x, c.y))
                .or_insert(HashSet::new())
                .insert(c.cart_id);
        }
    });

    carts.sort_by(|c1, c2| (c1.y, c1.x).cmp(&(c2.y, c2.x)));
    carts
        .iter_mut()
        .for_each(|c| {
            if !removed.contains(&c.cart_id) {
                let mut crashed = false;

                positions.get_mut(&(c.x, c.y)).unwrap().remove(&c.cart_id);

                c.tick(&track_rows);
                {
                    let mut option = positions.get_mut(&(c.x, c.y));
                    match option {
                        Some(ref mut a) => {
                            if a.len() != 0 {
                                crashed = true;
                                a
                                    .iter()
                                    .for_each(|id| {
                                        removed.insert(*id);
                                    });
                                a.clear();
                                removed.insert(c.cart_id);
                            }
                        },
                        _ => {}
                    }
                }

                if !crashed
                {
                    positions
                        .entry((c.x, c.y))
                        .or_insert(HashSet::new())
                        .insert(c.cart_id);
                }
            }
        });
}

fn to_string(track: Track) -> String {
    String::from(match track {
        Track::Across => "-",
        Track::Vertical => "|",
        Track::ForwardDiagonal => "/",
        Track::BackwardDiagonal => "\\",
        Track::Intersection => "+"
    })
}

fn print_world(track_rows: &Vec<Vec<Option<Track>>>, carts: &Vec<Cart>) {
    for j in 0..track_rows.len() {
        for i in 0..track_rows[j].len() {
            let option =
                carts.iter().find(|c| c.x == i && c.y == j);
            match option {
                None => {
                    match track_rows[j][i] {
                        Some(t) => {
                            print!("{}", to_string(t));
                        }
                        None => {
                            print!(" ");
                        }
                    }
                },
                Some(cart) => {
                    match cart.dir {
                        Direction::Up => {
                            print!("^");
                        },
                        Direction::Down => {
                            print!("v")
                        },
                        Direction::Left => {
                            print!("<")
                        },
                        Direction::Right => {
                            print!(">")
                        },
                    }
                },
            }
        }
        println!("");
    }
}