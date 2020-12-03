use std::fs;

const TREE_CHAR: &str = "#";

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut right_count = 3;
    let mut down_count = 1;
    let mut line_index = 0;
    let mut tree_count = 0;
    input.lines().for_each(|line| {
        // println!("{}", line);
        if line_index == down_count {
            let c = line
                .chars()
                .nth(right_count)
                .and_then(|c| Some(c.to_string()));

            match c {
                Some(c) => {
                    println!("{}", c);
        
                    if c == TREE_CHAR {
                        tree_count += 1;
                    }
                }
                None => ()
            }

            down_count += 1;
            right_count += 3;
        }

        // let char

        line_index += 1;
    });
    println!("{}", tree_count);
}
