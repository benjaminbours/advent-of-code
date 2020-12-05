use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

const YEAR: u32 = 2020;

fn main() {
    first_iteration();
    first_part_functional();
    second_part_functional();
}

fn first_iteration() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let mut result: Option<u32> = None;
    'outer: for line in &lines {
        let number_one = line.parse::<u32>().unwrap();
        for l in &lines {
            let number_two = l.parse::<u32>().unwrap();
            for i in &lines {
                let number_three = i.parse::<u32>().unwrap();
                let sum = number_one + number_two + number_three;
                if sum == YEAR {
                    println!(
                        "The three number the sum equal {} are {}, {} and {}",
                        YEAR, number_one, number_two, number_three
                    );
                    result = Some(number_one * number_two * number_three);
                    break 'outer;
                }
            }
        }
    }

    println!("These three number multiplied equal {}", result.unwrap());
}

fn first_part_functional() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| x.parse::<u32>().unwrap());
    let entries = HashSet::<u32>::from_iter(lines);

    let result = entries
        .iter()
        .find_map(|x| entries.get(&(YEAR - x)).map(|y| y * x));

    if result.is_some() {
        println!("first part result is {}", result.unwrap());
    } else {
        println!("no result");
    }
}

fn second_part_functional() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().map(|x| x.parse::<i32>().unwrap());
    let entries = HashSet::<i32>::from_iter(lines);

    let result = entries.iter().find_map(|x| {
        entries
            .iter()
            .filter(|&y| x != y)
            .find_map(|y| entries.get(&(YEAR as i32 - x - y)).map(|z| x * y * z))
    });

    if result.is_some() {
        println!("second part result is {}", result.unwrap());
    } else {
        println!("no result");
    }
}
