use std::fs;

const YEAR: u32 = 2020;

fn main() {
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
