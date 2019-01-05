extern crate regex;

#[derive(Debug)]
struct Player
{
    score: usize
}

struct Entry
{
    value: usize,
    prev: usize,
    next: usize,
//    idx: usize
}
//
//fn get(storage: &mut Vec<Entry>, idx: usize) -> &mut Entry {
//    storage.get_mut(idx).unwrap()
//}

fn main() {
    let player_count = 448;
    let marble_count= 7162800;

    let mut players = Vec::new();
    for _i in 0..player_count {
        players.push(Player {score: 0});
    }

    let mut storage : Vec<Entry> = Vec::with_capacity(marble_count);

    storage.push(Entry { value: 0, prev: 0, next: 0 });
    let mut current_entry_index : usize = 0;

    for j in 1..marble_count {
        let mut current_player = players.get_mut(j % player_count).unwrap();

        if j % 23 == 0
        {
            current_player.score += j;

            let mut skip_count = 0;
            while skip_count < 7 {
                current_entry_index = storage.get_mut(current_entry_index).unwrap().prev;
                skip_count += 1;
            }

            let mut previous;
            let mut next;
            {
                let mut cur = storage.get_mut(current_entry_index).unwrap();
                previous = cur.prev;
                next = cur.next;
                current_player.score += cur.value;
            }

            storage.get_mut(previous).unwrap().next = next;
            storage.get_mut(next).unwrap().prev = previous;
            current_entry_index = next;

        } else {
            let mut skip_count = 0;
            while skip_count < 2 {
                current_entry_index = storage.get_mut(current_entry_index).unwrap().next;
                skip_count += 1;
            }

            let new_index = storage.len();
            let mut new_entry_previous;
            {
                let mut entry_after_new =
                    storage.get_mut(current_entry_index).unwrap();
                new_entry_previous = entry_after_new.prev;
                entry_after_new.prev = new_index;
            }

            storage.push(
                Entry {
                    value: j,
                    prev: new_entry_previous,
                    next: current_entry_index
                });

            storage.get_mut(new_entry_previous).unwrap().next = new_index;
            current_entry_index = new_index;
        }
    }


    players.sort_by(|p1, p2| p2.score.cmp(&p1.score));
    println!("{:?}", players);
}