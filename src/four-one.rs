#[macro_use]
extern crate nom;
extern crate chrono;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Add;

use chrono::prelude::*;
use nom::{is_digit};

#[derive(Debug, Copy, Clone)]
enum Entry {
    BeginShift(u32),
    Sleep,
    Awake
}

#[derive(Debug, Copy, Clone)]
struct TimedEntry {
    time: DateTime<Utc>,
    entry: Entry
}

//named!(james, do_parse!(take!(2) >> ("foo")));
named!(textual_number <u32>, do_parse!(digs: take_while!(is_digit) >> (buf_to_u32(digs))));
named!(take_4_digits, take!(4));
named!(take_2_digits, take!(2));
named!(year <i32>, do_parse!(year: call!(take_4_digits) >> (buf_to_i32(year))));
named!(twoth <u32>, do_parse!( piece: take_2_digits >> ( buf_to_u32(piece) )));
named!(isoish_time <DateTime<Utc>>, do_parse!(
    tag!("[") >>
    y: year >>
    tag!("-") >>
    m: twoth >>
    tag!("-") >>
    d: twoth >>
    tag!(" ") >>
    h: twoth >>
    tag!(":") >>
    mins: twoth >>
    tag!("]") >>
    ( Utc.ymd(y, m, d).and_hms(h, mins, 0) )
));
named!(asleep <Entry>, do_parse!(tag!("falls asleep") >> (Entry::Sleep)));
named!(awake <Entry>, do_parse!(tag!("wakes up") >> (Entry::Awake)));
named!(shift_start <Entry>, do_parse!(
    tag!("Guard #") >>
    id: textual_number >>
    tag!(" begins shift") >>
    (Entry::BeginShift(id))
));
named!(entry <Entry>,
    alt!(
        asleep | awake | shift_start
    )
);
named!(timed_entry <TimedEntry>, do_parse!(
    time: isoish_time >> tag!(" ") >> ent: entry >> (TimedEntry { time: time, entry: ent })
));

fn parse_entry(line: &str) -> TimedEntry
{
    let result = timed_entry(line.as_bytes());
    result.unwrap().1
}

#[derive(Debug, Copy, Clone)]
enum GuardState
{
    Awake,
    Asleep(DateTime<Utc>)
}

#[derive(Debug, Copy, Clone)]
struct Guard
{
    id: u32,
    state: GuardState
}

fn inc_mins(start: DateTime<Utc>, end: DateTime<Utc>, out: &mut HashMap<u32, u64>)
{
    let mut it = start;
    while it != end
    {
        let e = out.entry(it.minute()).or_insert(0);
        *e += 1;
        it = it.add(chrono::Duration::minutes(1));
    }
}

#[derive(Debug, Copy, Clone)]
struct Winner {
    total: u64,
    max_minute: u32,
    max_asleep: u64,
    guard_id: u32
}

fn compute_score(guard_id: u32, sleep: &HashMap<u32, u64>) -> Winner
{
    let mut max_minute = 0;
    let mut max_asleep = 0;
    let mut total = 0;
    sleep.iter().for_each(|(k, v)| {
        if *v > max_asleep {
            max_asleep = *v;
            max_minute = *k;
        }
        total += *v;
    });

    Winner { total, max_minute, max_asleep, guard_id }
}

fn main() {
    let f = File::open("src/day4input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut rs: Vec<TimedEntry> = file.lines().map(|l| parse_entry(&l.unwrap())).collect();

    rs.sort_by(|a, b| a.time.cmp(&b.time));

    let mut a: Option<Guard> = Option::None;
    let mut result = HashMap::new();

    rs.iter().for_each(|r| {
        a = match (a, r.entry) {
            (Option::None, Entry::BeginShift(id)) => {
                Option::Some(Guard { id, state: GuardState::Awake })
            },
            (Option::Some(g), Entry::BeginShift(id)) => {
                match g.state {
                    GuardState::Awake => {
                        Option::Some(Guard { id, state: GuardState::Awake })
                    }
                    GuardState::Asleep(dt) => {
                        inc_mins(dt, r.time, result.entry(g.id).or_insert_with(HashMap::new));
                        Option::Some(Guard { id: g.id, state: GuardState::Awake })
                    }
                }
            },
            (Option::Some(g), Entry::Sleep) => {
                Option::Some( Guard { id: g.id, state: GuardState::Asleep(r.time) })
            }
            (Option::Some(g), Entry::Awake) => {
                match g.state {
                    GuardState::Asleep(dt) => {
                        inc_mins(dt, r.time, result.entry(g.id).or_insert_with(HashMap::new));
                        Option::Some( Guard { id: g.id, state: GuardState::Awake })
                    }
                    _ => a
                }
            }
            _ => a
        };
    });

    let win = result.iter().fold(Option::None, | a : Option<Winner>, (k, v) | {
        let w = compute_score(*k, v);
        match a {
            Option::Some(a) => {
                if w.total > a.total {
                    Option::Some(w)
                } else {
                    Option::Some(a)
                }
            }
            _ => Option::Some(w)
        }
    });

    let win2 = result.iter().fold(Option::None, | a : Option<Winner>, (k, v) | {
        let w = compute_score(*k, v);
        match a {
            Option::Some(a) => {
                if w.max_asleep > a.max_asleep {
                    Option::Some(w)
                } else {
                    Option::Some(a)
                }
            }
            _ => Option::Some(w)
        }
    });

    println!("{:?}", win);
    println!("{:?}", win2);
}

use std::str::{FromStr, from_utf8_unchecked};
use std::collections::HashMap;

pub fn to_string(s: &[u8]) -> &str {
    unsafe { from_utf8_unchecked(s) }
}
pub fn to_i32(s: &str) -> i32 {
    FromStr::from_str(s).unwrap()
}
pub fn to_u32(s: &str) -> u32 {
    FromStr::from_str(s).unwrap()
}

pub fn buf_to_u32(s: &[u8]) -> u32 {
    to_u32(to_string(s))
}
pub fn buf_to_i32(s: &[u8]) -> i32 {
    to_i32(to_string(s))
}
