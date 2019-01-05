extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::VecDeque;

use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

impl Debug for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Worker::Idle => f.write_str("."),
            Worker::WorkingOn(item) => f.write_str(&item.name)
        }
    }
}

#[derive(Clone)]
enum Worker
{
    Idle,
    WorkingOn(WorkItem)
}

impl Debug for WorkItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{}, ({})", &self.name, self.time_required))
    }
}

#[derive(Clone)]
struct WorkItem
{
    name: String,
    time_required: u32
}

#[derive(Debug)]
struct WorkQueue
{
    workers: Vec<Worker>,
    queue: VecDeque<WorkItem>
}

// add an item to a work queue
fn schedule_work(work_queue: &mut WorkQueue, work_item: WorkItem) {
    work_queue.queue.push_back(work_item);
}

fn progress(item: &WorkItem) -> WorkItem {
    if item.name == "G"
    {
        println!("{:?}", item);
    }
    WorkItem {
        time_required: item.time_required - 1,
        name: item.name.clone()
    }
}

fn is_idle(w: &Worker) -> bool {
    match w {
        Worker::Idle => true,
        _ => false
    }
}

fn idle(work_queue: &WorkQueue) -> bool {
    work_queue.queue.is_empty() &&
        work_queue.workers.iter().all(|w| is_idle(w))
}

fn assign_work(work_queue: &WorkQueue) -> WorkQueue {
    let mut new_workers = Vec::new();
    let mut new_queue = work_queue.queue.clone();

    work_queue.workers.iter().for_each(|w| {
        match w {
            Worker::WorkingOn(item) => {
                new_workers.push(Worker::WorkingOn(item.clone()));
            }
            _ => {
                let next_item = new_queue.pop_front();
                match next_item {
                    Some(item) => {
                        new_workers.push(Worker::WorkingOn(item))
                    }
                    _ => {
                        new_workers.push(Worker::Idle);
                    }
                }
            }
        }
    });

    WorkQueue{ queue: new_queue, workers: new_workers }
}

// do one second's worth of work on a work queue
// return any complete item names
fn tick(
    work_queue: &WorkQueue,
    deps: &mut HashMap<String, HashSet<String, RandomState>, RandomState>,
    reverse_deps: &HashMap<String, Vec<String>, RandomState>)
    -> WorkQueue
{
    let mut new_workers = Vec::new();
    let mut new_queue = work_queue.queue.clone();

    work_queue.workers.iter().for_each(|w| {
        match w {
            Worker::WorkingOn(item) => {
                if item.time_required == 1
                {
                    complete_item(deps, reverse_deps, &item.name);
                    let new_items: Vec<String> = new_work_items(deps);
                    new_items
                        .iter()
                        .for_each(|i| {
                            new_queue.push_back(work_item(i));
                            deps.remove(i);
                        });

                    new_workers.push(Worker::Idle);
                }
                else
                {
                    new_workers.push(Worker::WorkingOn(progress(item)));
                }
            },
            Worker::Idle => {
                new_workers.push(Worker::Idle);
            }
        }
    });
    let queue = WorkQueue { workers: new_workers, queue: new_queue };
    assign_work(&queue)
}

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

    println!("part one");
    part_one(&deps, &reverse_deps);

    println!("");
    println!("part two");
    part_two(&deps, &reverse_deps);
}

fn work_item(name: &str) -> WorkItem
{
    let required = 61 + name.as_bytes()[0] as u32 - 'A' as u32;
    let res = WorkItem { name: name.to_string(), time_required: required };
    println!("{:?}", res);
    res
}

fn part_two(
    deps_base: &HashMap<String, HashSet<String, RandomState>, RandomState>,
    reverse_deps_base: &HashMap<String, Vec<String>, RandomState>) {
    let mut deps = deps_base.clone();
    let reverse_deps = reverse_deps_base.clone();

    let mut work_queue = WorkQueue { workers: Vec::new(), queue: VecDeque::new() };
    work_queue.workers.push(Worker::Idle);
    work_queue.workers.push(Worker::Idle);
    work_queue.workers.push(Worker::Idle);
    work_queue.workers.push(Worker::Idle);
    work_queue.workers.push(Worker::Idle);
    let done = new_work_items(&mut deps);
    done.iter().for_each(
        |i| {
            schedule_work(&mut work_queue, work_item(i));
            deps.remove(i);
        });
    work_queue = assign_work(&work_queue);

    let mut time_taken = 0;
    while !idle(&work_queue)
    {
        // last guess: PQFWKJSVXYUEMZDNIHTAGOCRLB
        //             PQWFKJSVXYUEMZDNIHTAGOCRLB
        println!("{:03}, {:?}", time_taken, work_queue);
        work_queue = tick(&work_queue, &mut deps, &reverse_deps);
        time_taken += 1;
    }

    println!("{}", time_taken);
}

fn part_one(
    deps_base: &HashMap<String, HashSet<String, RandomState>, RandomState>,
    reverse_deps_base: &HashMap<String, Vec<String>, RandomState>) {

    let mut deps = deps_base.clone();
    let reverse_deps = reverse_deps_base.clone();

    while !deps.is_empty() {
        let done = new_work_items(&mut deps);

        if !done.is_empty() {
            let k = done[0].clone();
            deps.remove(&k);
            complete_item(&mut deps, &reverse_deps, &k)
        }
    }
}

fn complete_item(
    deps: &mut HashMap<String, HashSet<String, RandomState>, RandomState>,
    reverse_deps: &HashMap<String, Vec<String>, RandomState>,
    k: &String) {
    //print!("{}", k);
    //println!("{} completed", k);
    let option = reverse_deps.get(k);
    match option {
        Some(vec) => {
            vec.iter().for_each(|k2| {
                let option_two = deps.get_mut(k2);
                match option_two {
                    Some(set) => {
                        set.remove(k);
                    },
                    _ => {}
                }
            })
        },
        _ => {}
    }
}

fn new_work_items(deps: & HashMap<String, HashSet<String, RandomState>, RandomState>)
    -> Vec<String> {
    let mut done: Vec<String> = Vec::new();
    deps.iter().for_each(|(k, v)| {
        if v.is_empty() {
            done.push(k.clone());
        }
    });
    done.sort();
    done
}
