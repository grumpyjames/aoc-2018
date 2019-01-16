use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("src/day5input.txt").unwrap();
    let mut input: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    f.read_to_end(&mut input).unwrap();

    println!("{}", input.len());

    let mut bitmask : Vec<bool> = Vec::with_capacity(input.len());
    for _i in 0..input.len() {
        bitmask.push(true);
    }

    let cond = |l, r| l + 32 == r || r + 32 == l;

    run(&mut input, &mut bitmask, 0, 1, cond);

    let mut count = 0;
    for i in 0..input.len() {
        if bitmask[i] {
            print!("{}", input[i] as char);
            count += 1;
        }
    }
    println!("");
    println!("{}", count);

    let mut best_count = 50000;
    for c in 65..91 {
        let mut bitmask : Vec<bool> = Vec::with_capacity(input.len());
        for i in 0..input.len() {
            bitmask.push(input[i] != c && c + 32 != input[i])
        }

        let mut left_index = 0;
        while left_index < input.len() && !bitmask[left_index] {
            left_index += 1;
        }
        let mut right_index = left_index;
        while right_index < input.len() && !bitmask[right_index] {
            right_index += 1;
        }

        run(&mut input, &mut bitmask, left_index, right_index, cond);
        let mut count = 0;
        for i in 0..input.len() {
            if bitmask[i] {
                count += 1;
            }
        }

        if (count < best_count) {
            best_count = count;

            println!("");
            println!("BESTEST {}, {}", c as char, count);
        }
    }
}

fn run(
    input: &mut Vec<u8>,
    bitmask: &mut Vec<bool>,
    start_left_index: usize,
    start_right_index: usize,
    cond: fn(u8, u8) -> bool) {

    let mut left_index = start_left_index;
    let mut right_index = start_right_index;

    while right_index < input.len() {
        let left = input[left_index];
        let right = input[right_index];
        if cond(left, right) {
                // omit the left and right characters
            bitmask[left_index] = false;
            bitmask[right_index] = false;
            while left_index > 0 && !bitmask[left_index] {
                left_index -= 1;
            }
            while right_index < input.len() && !bitmask[right_index] {
                right_index += 1;
            }
        } else {
            left_index = right_index;
            right_index += 1;
            while right_index < input.len() && !bitmask[right_index] {
                right_index += 1;
            }
        }
    }
}

