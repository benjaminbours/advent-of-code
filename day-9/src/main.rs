use std::fs;

const MARGE: usize = 25;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let numbers: Vec<i64> = lines.map(|line| line.parse::<i64>().unwrap()).collect();

    // numbers.iter().for_each(||)

    let mut index = 0;
    loop {
        // do nothing the first 25 numbers
        if index <= 24 {
            index += 1;
            continue;
        }
        let number = numbers.get(index);

        match number {
            Some(n) => {
                let previous_numbers = &numbers[(index - MARGE)..index].to_owned();

                let result = is_valid(n, previous_numbers);

                if result == false {
                    println!("{}, previous: {:?}", n, previous_numbers);
                    println!("{}", result);
                }
            }
            None => {
                break;
            }
        }

        index += 1;
    }
}

fn is_valid(sum: &i64, previous_numbers: &Vec<i64>) -> bool {
    previous_numbers.iter().any(|x| {
        previous_numbers
            .iter()
            .find(|y| (&x != y) && x.clone() + y.clone() == sum.clone())
            .is_some()
    })
}
