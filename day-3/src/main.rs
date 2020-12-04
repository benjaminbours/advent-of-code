use colored::*;
use std::fs;

const TREE_CHAR: &str = "#";

struct Slope {
    right: i32,
    down: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let results: i64 = slopes
        .iter()
        .map(|slope| {
            let mut right_count: i32 = 0;
            let mut down_count = slope.down;
            let mut tree_count = 0;
            let mut line_index = 0;

            input.lines().for_each(|line| {
                let line_length = line.chars().count();

                if line_index == down_count {
                    right_count += slope.right;
                    if right_count >= line_length as i32 {
                        let line_rest = right_count - line_length as i32;
                        right_count = line_rest;
                    }

                    let chart = {
                        let mut chars = line.chars();
                        chars
                            .nth(right_count as usize)
                            .and_then(|c| Some(c.to_string()))
                            .unwrap()
                    };

                    if chart == TREE_CHAR {
                        tree_count += 1;
                    }

                    down_count += slope.down;
                }
                line_index += 1;
            });
            println!("{}", tree_count);

            tree_count
        })
        .fold(
            0,
            |acc, item| {
                if acc == 0 {
                    acc + item
                } else {
                    acc * item
                }
            },
        );
        // use product instead of fold

    println!("{:?}", results);
}

fn print_line(line: &str, chart: &String, char_index: i32) {
    if char_index == 0 {
        println!("{}{}", chart.red(), &line[(char_index + 1) as usize..]);
    } else {
        println!(
            "{}{}{}",
            &line[..=(char_index - 1) as usize],
            chart.red(),
            &line[(char_index + 1) as usize..]
        );
    }
}
