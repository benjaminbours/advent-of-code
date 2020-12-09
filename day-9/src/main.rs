use std::fs;

const MARGE: usize = 25;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let numbers: Vec<i64> = lines.map(|line| line.parse::<i64>().unwrap()).collect();

    let mut index = 0;
    let invalid_number = loop {
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
                    break (Some(n.to_owned()));
                }
            }
            None => {
                break (None);
            }
        }

        index += 1;
    }
    .unwrap();

    numbers.iter().enumerate().for_each(|(index_start, n)| {
        let mut acc: Vec<&i64> = vec![];

        let numbers_to_accumulate = &numbers[index_start..];

        let mut index = 0;
        let mut index_end = 0;
        numbers_to_accumulate.iter().for_each(|x| {
            acc.push(x);

            let current_count: i64 = acc.clone().into_iter().sum();
            if index > 1 && (current_count == invalid_number) {
                index_end = index_start + index;
                println!("HERE start: {}, end: {}", index_start, index_end);

                let smallest = acc.iter().min().unwrap().clone();
                let biggest = acc.iter().max().unwrap().clone();

                println!("range: {:?}", acc);
                println!("result: {}", smallest + biggest);
            }

            index += 1;
        });
    });
}

fn is_valid(sum: &i64, previous_numbers: &Vec<i64>) -> bool {
    previous_numbers.iter().any(|x| {
        previous_numbers
            .iter()
            .find(|y| (&x != y) && x.clone() + y.clone() == sum.clone())
            .is_some()
    })
}
