extern crate regex;

struct FuelCell
{
    x: usize,
    y: usize
}

fn third_digit(input: usize) -> u8 {
    ((input / 100) % 10) as u8
}

impl FuelCell
{
    fn rack_id(&self) -> usize
    {
        self.x + 10
    }

    fn initial_power(&self) -> usize
    {
        self.rack_id() * self.y
    }

    fn compute_power_level(&self, serial_no: usize) -> i8
    {
        (third_digit((self.initial_power() + serial_no)* self.rack_id()) as i8) - 5
    }
}




fn main() {
//    let serial_no = 8;
//    let grid_size = 5;
    let serial_no = 9221;
    let grid_size = 300;
    let mut power_grid_rows = Vec::with_capacity(grid_size);
    for j in 0..grid_size {
        let mut row = Vec::with_capacity(grid_size);
        for i in 0..grid_size {
            let power = FuelCell { x: i + 1, y: j + 1 }.compute_power_level(serial_no);
            row.push(power);
        }
        power_grid_rows.push(row);
    }

//    for j in 59..64 {
//        for i in 20..26 {
//            let pwr = power_grid_rows[j][i];
//            if pwr < 0 {
//                print!("{} ", pwr);
//            } else {
//                print!(" {} ", pwr);
//            }
//        }
//        println!("")
//    }

    let mut best = 0;
    for l in 1..=(grid_size - 1) {
        for j in 0..=(grid_size - l) {
            for i in 0..=(grid_size - l) {
                let mut cell_power : i16 = 0;

                for m in 0..l {
                    for n in 0..l {
                        cell_power += power_grid_rows[j + m][i + n] as i16
                    }
                }

                if cell_power > best {
                    best = cell_power;
                    println!("{}, {}, {}, {}", i, j, l, best);
                }
            }
        }
    }
}