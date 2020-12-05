use std::fs;

const MAX_ROWS: i32 = 127;
const MAX_COLUMNS: i32 = 7;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let seats = input.lines().map(|line| {
        let row = &line[..7]
            .chars()
            .fold((0, MAX_ROWS), |acc, x| {
                let (min, max) = acc;
                match x {
                    'F' => (min, ((max - min) / 2) + min),
                    'B' => ((max - min) / 2 + min + 1, max),
                    _ => acc,
                }
            })
            .0;

        // println!("row: {:?}", row);

        let column = &line[7..]
            .chars()
            .fold((0, MAX_COLUMNS), |acc, x| {
                // println!("{}, acc: {:?}", x, acc);
                let (min, max) = acc;
                match x {
                    'R' => ((max - min) / 2 + min + 1, max),
                    'L' => (min, ((max - min) / 2) + min),
                    _ => acc,
                }
            })
            .0;

        // println!("column: {:?}", column);

        let seat_id = row * 8 + column;

        seat_id

    });

    println!("{:?}", seats.clone().collect::<Vec<i32>>());

    let highest_seat_id = seats.max().unwrap();

    println!("{:?}", highest_seat_id);
}
