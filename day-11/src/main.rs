use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug, Copy, Clone)]
enum SeatState {
    Empty,
    Occupied,
    Floor,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Vector2 {
    x: i32,
    y: i32,
}

const FLOOR_CHAR: char = '.';
const EMPTY_SEAT_CHAR: char = 'L';
const OCCUPIED_SEAT_CHAR: char = '#';

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut grid: HashMap<Vector2, SeatState> =
        lines
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row, line)| {
                let seats = line.chars();

                seats.enumerate().for_each(|(column, seat)| {
                    let state = match seat {
                        FLOOR_CHAR => SeatState::Floor,
                        EMPTY_SEAT_CHAR => SeatState::Empty,
                        OCCUPIED_SEAT_CHAR => SeatState::Occupied,
                        _ => SeatState::Floor,
                    };
                    acc.insert(
                        Vector2 {
                            x: column as i32,
                            y: row as i32,
                        },
                        state,
                    );
                });

                acc
            });

    let mut count = grid
        .iter()
        .filter(|(_, state)| state == &&SeatState::Occupied)
        .count();
    loop {
        round(&mut grid);
        let last_count = grid
            .iter()
            .filter(|(_, state)| state == &&SeatState::Occupied)
            .count();
        if count == last_count {
            count = last_count;
            break;
        } else {
            count = last_count;
        }
    }

    println!("{:?}", count);
    // println!("{:?}", last_grid.co);
}

fn round(grid: &mut HashMap<Vector2, SeatState>) {
    let initial_grid = grid.clone();
    grid.iter_mut().for_each(|(position, state)| {
        let adjacent_seats = find_adjacent_seats(position, &initial_grid);
        // println!("adj seats: {}", adjacent_seats);
        match state {
            SeatState::Empty if adjacent_seats == 0 => *state = SeatState::Occupied,
            SeatState::Occupied if adjacent_seats >= 5 => *state = SeatState::Empty,
            _ => (),
        }
    });
}

fn find_adjacent_seats<'a>(position: &Vector2, grid: &'a HashMap<Vector2, SeatState>) -> i32 {
    let directions = vec![
        (1, 0),   // right
        (-1, 0),  // left
        (0, -1),  // top
        (0, 1),   // bottom
        (1, 1),   // bottom-right
        (-1, 1),  // bottom-left
        (-1, -1), // top-left
        (1, -1),  // top-right
    ];

    let adjacent_seats: Vec<(Vector2, &SeatState)> =
        directions.iter().fold(vec![], |mut acc, direction| {
            match find_seat_in_direction(position, direction, grid) {
                Ok(s) => acc.push(s),
                Err(_) => (),
            }
            acc
        });

    adjacent_seats.iter().count() as i32
}

fn find_seat_in_direction<'a>(
    start: &Vector2,
    (x, y): &(i32, i32),
    grid: &'a HashMap<Vector2, SeatState>,
) -> Result<(Vector2, &'a SeatState), String> {
    let vector = Vector2 {
        x: start.x + x,
        y: start.y + y,
    };

    match grid.get(&vector) {
        Some(seat_state) => match seat_state {
            SeatState::Occupied => Ok((vector, seat_state)),
            SeatState::Empty => Err("Can only see empty seat in this direction".to_string()),
            SeatState::Floor => find_seat_in_direction(&vector, &(x.clone(), y.clone()), grid),
        },
        None => Err("nothing in this direction".to_string()),
    }
}
