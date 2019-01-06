extern crate regex;

use std::collections::HashMap;


fn main() {
    let mut state =
        //String::from(/* ..... */"#..#.#..##......###...###" /* ..... */);
        String::from("######....##.###.#..#####...#.#.....#..#.#.##......###.#..##..#..##..#.##..#####.#.......#.....##..");
    let mut evolute = HashMap::new();

//
//    evolute.insert("...##".to_string(), "#".to_string());
//    evolute.insert("..#..".to_string(), "#".to_string());
//    evolute.insert(".#...".to_string(), "#".to_string());
//    evolute.insert(".#.#.".to_string(), "#".to_string());
//    evolute.insert(".#.##".to_string(), "#".to_string());
//    evolute.insert(".##..".to_string(), "#".to_string());
//    evolute.insert(".####".to_string(), "#".to_string());
//    evolute.insert("#.#.#".to_string(), "#".to_string());
//    evolute.insert("#.###".to_string(), "#".to_string());
//    evolute.insert("##.#.".to_string(), "#".to_string());
//    evolute.insert("##.##".to_string(), "#".to_string());
//    evolute.insert("###..".to_string(), "#".to_string());
//    evolute.insert("###.#".to_string(), "#".to_string());
//    evolute.insert("####.".to_string(), "#".to_string());
//
    evolute.insert("...##".to_string(), "#".to_string());
    evolute.insert("###..".to_string(), ".".to_string());
    evolute.insert("#.#.#".to_string(), ".".to_string());
    evolute.insert("#####".to_string(), ".".to_string());
    evolute.insert("....#".to_string(), ".".to_string());
    evolute.insert("##.##".to_string(), ".".to_string());
    evolute.insert("##.#.".to_string(), "#".to_string());
    evolute.insert("##...".to_string(), "#".to_string());
    evolute.insert("#..#.".to_string(), "#".to_string());
    evolute.insert("#.#..".to_string(), ".".to_string());
    evolute.insert("#.##.".to_string(), ".".to_string());
    evolute.insert(".....".to_string(), ".".to_string());
    evolute.insert("##..#".to_string(), ".".to_string());
    evolute.insert("#..##".to_string(), ".".to_string());
    evolute.insert(".##.#".to_string(), "#".to_string());
    evolute.insert("..###".to_string(), "#".to_string());
    evolute.insert("..#.#".to_string(), "#".to_string());
    evolute.insert(".####".to_string(), "#".to_string());
    evolute.insert(".##..".to_string(), ".".to_string());
    evolute.insert(".#..#".to_string(), "#".to_string());
    evolute.insert("..##.".to_string(), ".".to_string());
    evolute.insert("#....".to_string(), ".".to_string());
    evolute.insert("#...#".to_string(), ".".to_string());
    evolute.insert(".###.".to_string(), ".".to_string());
    evolute.insert("..#..".to_string(), ".".to_string());
    evolute.insert("####.".to_string(), "#".to_string());
    evolute.insert(".#.##".to_string(), ".".to_string());
    evolute.insert("###.#".to_string(), ".".to_string());
    evolute.insert("#.###".to_string(), "#".to_string());
    evolute.insert(".#...".to_string(), "#".to_string());
    evolute.insert(".#.#.".to_string(), ".".to_string());
    evolute.insert("...#.".to_string(), ".".to_string());


    // .#....##....#####...#######....#.#..##.
    // .#....##....#####...#######....#.#..##.

    println!("{}", state);
    for _h in 0..20 {
        let mut next_gen = String::new();
        for i in 0..state.len()+10 {
            let mut window = String::new();
            for j in i..i + 5 {
                let k = (j as i32) - 7;
                if 0 <= k && k < (state.len() as i32) {
                    window += &state[k as usize..(k + 1) as usize];
                } else {
                    window += "."
                }
            }
            //println!("{}", window);
            next_gen += evolute.get(&window).unwrap_or(&String::from("."));
        }
        println!("{}", next_gen);
        state = next_gen.clone();
    }

    let bytes = state.as_bytes();
    let mut pot_index = -100;
    let mut total = 0;
    for i in 0..bytes.len() {
        if bytes[i] == '#' as u8 {
            total += pot_index;
        }
        pot_index += 1;
    }

    println!("{}", total);
}