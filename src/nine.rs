extern crate regex;

use std::collections::LinkedList;

#[derive(Debug)]
struct Player
{
    score: usize
}

fn main() {
    let player_count = 448;
    let marble_count= 7162800;

    let mut players = Vec::new();
    for _i in 0..player_count {
        players.push(Player {score: 0});
    }

    let mut circle : Vec<usize> = Vec::new();
    let mut current_index : usize = 0;
    circle.push(0);


    for j in 1..marble_count {
        let mut current_player = players.get_mut(j % player_count).unwrap();

        if j % 23 == 0
        {
            current_player.score += j;
            current_index = if current_index >= 7 {
                current_index - 7
            } else {
                circle.len() - 7 + current_index
            };
            let removed = circle.remove(current_index as usize);
            current_player.score += removed;
        }
        else
        {
            current_index = (current_index + 2) % circle.len();
            circle.insert(current_index as usize,j);
        }
    }

    players.sort_by(|p1, p2| p2.score.cmp(&p1.score));
    println!("{:?}", players);
}