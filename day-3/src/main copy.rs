use std::fs;

const TREE_CHAR: &str = "#";

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    // lines
    let mut right_count = 3;
    // chars
    let mut down_count = 1;
    let mut line_index = 0;
    let mut tree_count = 0;
    input.lines().for_each(|line| {
        let chars = line.chars();
        let chars_count = &chars.count();

        
        if line_index == right_count {
            // if &down_count > chars_count {
            //     return;
            // }
            // println!("{}", chars_count);
            let index = chars_count - down_count - 1;
            let yo = 0;
            if index < 0 {
                return;
            }
            println!("{}", line);
            // println!("{}", index);
            let c = line.chars().nth(index as usize).and_then(|c| Some(c.to_string()));

            match c {
                Some(c) => {
                    println!("{}, {}", line_index, index);
                    println!("{}", c);

                    if c == TREE_CHAR {
                        tree_count += 1;
                    }
                }
                None => (),
            }

            down_count += 1;
            right_count += 3;
        }

        line_index += 1;
    });
    println!("{}", tree_count);
}
