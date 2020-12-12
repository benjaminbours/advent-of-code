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
    value: i32,
    action: Actions,
}

#[derive(Debug, Default)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let instructions = load_instructions(input);
    let initial_direction = Directions::East;
    let coordinates = follow_instructions(instructions, initial_direction);
    println!("answer: {}", coordinates.x + coordinates.y);
}

fn load_instructions(input: String) -> Vec<Instruction> {
    let lines: Vec<&str> = input.lines().collect();

    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|line| {
            let first = &line[0..1];
            let value = line[1..].trim().parse::<i32>().unwrap();

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

fn follow_instructions(instructions: Vec<Instruction>, initial_direction: Directions) -> Coordinates {
    let mut index = 0;
    let mut current_direction = initial_direction;
    let coordinates = instructions
        .iter()
        .fold(Coordinates::default(), |mut acc, instruction| {
            println!("{}", index);
            match &instruction.action {
                Actions::Forward => {
                    update_coordinates(&mut acc, &current_direction, instruction.value)
                }
                Actions::Left => update_direction(&mut current_direction, instruction.value),
                Actions::Right => update_direction(&mut current_direction, -instruction.value + 360),
                Actions::Direction(direction) => {
                    update_coordinates(&mut acc, direction, instruction.value)
                }
            }

            index += 1;

            acc
        });

    println!("{:?}", coordinates);

    Coordinates {
        x: coordinates.x.abs(),
        y: coordinates.y.abs(),
    }
}

fn update_direction(direction: &mut Directions, rotation: i32) {
    let mut degrees = match direction {
        Directions::East => {
            let current_rotation = if rotation < 0 { 360 } else { 0 };
            (current_rotation + rotation).abs()
        }
        Directions::West => {
            let current_rotation = 180;
            (current_rotation + rotation).abs()
        }
        Directions::North => {
            let current_rotation = 90;
            (current_rotation + rotation).abs()
        }
        Directions::South => {
            let current_rotation = 270;
            (current_rotation + rotation).abs()
        }
    };

    if degrees > 360 {
        degrees -= 360;
    }

    let updated_direction = match degrees {
        0 | 360 => Directions::East,
        90 => Directions::North,
        180 => Directions::West,
        270 => Directions::South,
        _ => Directions::South,
    };

    // println!("initial {:?}", direction);
    println!("rotation {:?}, degrees: {}", rotation, degrees);
    println!("final {:?}", updated_direction);
    *direction = updated_direction;
}

fn update_coordinates(coordinates: &mut Coordinates, direction: &Directions, value: i32) {
    match direction {
        Directions::East => coordinates.x += value,
        Directions::West => coordinates.x -= value,
        Directions::North => coordinates.y += value,
        Directions::South => coordinates.y -= value,
    }
}

#[cfg(test)]
mod tests {
    use crate::{follow_instructions, load_instructions, Directions};

    #[test]
    fn it_works() {
        let input = "F10
N3
F7
R90
F11";
        let initial_direction = Directions::East;
        let instructions = load_instructions(input.to_string());
        let coordinates = follow_instructions(instructions, initial_direction);
        
        assert_eq!(25, coordinates.x + coordinates.y);
    }
}
