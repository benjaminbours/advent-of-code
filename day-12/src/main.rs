use std::f64::consts::PI;
use std::fs;

#[derive(Debug)]
enum Directions {
    North,
    South,
    East,
    West,
}
#[derive(Debug)]

enum Actions {
    Left,
    Right,
    Forward,
    Direction(Directions),
}

#[derive(Debug)]
struct Instruction {
    value: f64,
    action: Actions,
}

#[derive(Debug, Default)]
struct Coordinates {
    x: f64,
    y: f64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let instructions = load_instructions(input);
    let initial_way_point = Coordinates { x: 10., y: 1. };
    let initial_direction = Directions::East;
    let coordinates = follow_instructions(instructions, initial_direction, initial_way_point);
    println!("answer: {}", coordinates.x + coordinates.y);
}

fn load_instructions(input: String) -> Vec<Instruction> {
    let lines: Vec<&str> = input.lines().collect();

    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|line| {
            let first = &line[0..1];
            let value = line[1..].trim().parse::<f64>().unwrap();

            let action = match first {
                "N" => Actions::Direction(Directions::North),
                "S" => Actions::Direction(Directions::South),
                "E" => Actions::Direction(Directions::East),
                "W" => Actions::Direction(Directions::West),
                "L" => Actions::Left,
                "R" => Actions::Right,
                "F" => Actions::Forward,
                _ => Actions::Direction(Directions::East),
            };

            Instruction { action, value }
        })
        .collect();

    instructions
}

fn follow_instructions(
    instructions: Vec<Instruction>,
    initial_direction: Directions,
    initial_way_point: Coordinates,
) -> Coordinates {
    let mut way_point = initial_way_point;
    let coordinates = instructions
        .iter()
        .fold(Coordinates::default(), |mut acc, instruction| {
            match &instruction.action {
                Actions::Forward => {
                    update_ship_coordinates(&mut acc, instruction.value, &way_point);
                }
                Actions::Left => {
                    rotate_point(degrees_to_radian(instruction.value), &mut way_point);
                },
                Actions::Right => {
                    rotate_point(-degrees_to_radian(instruction.value), &mut way_point);
                }
                Actions::Direction(direction) => {
                    update_coordinates(&mut way_point, direction, instruction.value)
                }
            }

            acc
        });

    Coordinates {
        x: coordinates.x.abs(),
        y: coordinates.y.abs(),
    }
}

fn degrees_to_radian(degrees: f64) -> f64 {
    degrees as f64 * PI / 180.
}

fn rotate_point(angle: f64, point: &mut Coordinates) {
    let s = angle.sin();
    let c = angle.cos();
    println!("angle: {:?}", angle);
    println!("initial point: {:?}", point);

    
    let x = point.x;
    let y = point.y;
    // rotate point
    point.x = (x * c - y * s).round();
    point.y = (x * s + y * c).round();

    println!("rotated point: {:?}", point);
}

fn update_direction(direction: &mut Directions, rotation: f64) {
    let mut degrees = match direction {
        Directions::East => {
            let current_rotation = if rotation < 0. { 360. } else { 0. };
            (current_rotation + rotation).abs()
        }
        Directions::West => {
            let current_rotation = 180.;
            (current_rotation + rotation).abs()
        }
        Directions::North => {
            let current_rotation = 90.;
            (current_rotation + rotation).abs()
        }
        Directions::South => {
            let current_rotation = 270.;
            (current_rotation + rotation).abs()
        }
    };

    if degrees > 360. {
        degrees -= 360.;
    }

    let updated_direction = match degrees {
        0. | 360. => Directions::East,
        90. => Directions::North,
        180. => Directions::West,
        270. => Directions::South,
        _ => Directions::South,
    };

    // println!("initial {:?}", direction);
    // println!("rotation {:?}, degrees: {}", rotation, degrees);
    // println!("final {:?}", updated_direction);
    *direction = updated_direction;
}

fn update_coordinates(coordinates: &mut Coordinates, direction: &Directions, value: f64) {
    match direction {
        Directions::East => coordinates.x += value,
        Directions::West => coordinates.x -= value,
        Directions::North => coordinates.y += value,
        Directions::South => coordinates.y -= value,
    }
}

fn update_ship_coordinates(coordinates: &mut Coordinates, count: f64, way_point: &Coordinates) {
    coordinates.x += way_point.x * count;
    coordinates.y += way_point.y * count;
}

#[cfg(test)]
mod tests {
    use crate::{follow_instructions, load_instructions, Coordinates, Directions};

    #[test]
    fn it_works() {
        let input = "F10
N3
F7
R90
F11";
        let initial_way_point = Coordinates { x: 10., y: 1. };
        let initial_direction = Directions::East;
        let instructions = load_instructions(input.to_string());
        let coordinates = follow_instructions(instructions, initial_direction, initial_way_point);

        assert_eq!(286., coordinates.x + coordinates.y);
    }
}
