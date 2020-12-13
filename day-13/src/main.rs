use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let earliest_timestamp = lines.next().unwrap().parse::<i32>().unwrap();
    let buses = lines.next().unwrap();

    let result = find_earliest_timestamp(buses);
    println!("answer is: {}", result);
}

fn find_earliest_timestamp(input: &str) -> i64 {
    let buses: Vec<(usize, i32)> = input
        .split(',')
        .enumerate()
        .filter_map(|(index, x)| {
            let parsed = x.parse::<i32>();
            if parsed.is_ok() {
                return Some((index, parsed.unwrap()));
            }
            None
        })
        .collect();
    println!("{:?}", buses);

    let mut index: i64 = 0;
    let mut departures: Vec<(usize, i32)> = vec![];
    loop {
        buses.iter().for_each(|(i, bus_id)| {
            let is_departure_now =
                check_departure(&(index + i.clone() as i64), bus_id.to_owned() as i64);
            if is_departure_now {
                // println!("start at time: {}, {}", index, bus_id);
                departures.push((i.clone(), bus_id.clone()));
            }
        });

        let is_equal = vec_compare(&departures, &buses);

        // println!("check");
        // println!("{:?}", departures);
        // println!("{:?}", is_equal);

        if is_equal {
            println!("is equal: {} {:?}", is_equal, departures);
            break;
        } else {
            departures = vec![];
        }

        index += 1;
    }

    index
}

fn check_departure(timestamp: &i64, bus_id: i64) -> bool {
    let rest = timestamp % bus_id;
    rest == 0
}

fn vec_compare(va: &Vec<(usize, i32)>, vb: &Vec<(usize, i32)>) -> bool {
    (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| a.0 == b.0 && a.1 == b.1)
}

// fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
//     let prod: i64 = modulii.iter().product();

//     let sum = Option<i64> = residues.iter().zip(modulii)
// }

#[cfg(test)]
mod tests {
    use crate::find_earliest_timestamp;

    #[test]
    fn first_example() {
        let input = "67,7,59,61";
        let result = find_earliest_timestamp(input);
        assert_eq!(754018, result);
    }
    
    #[test]
    fn second_example() {
        let input = "67,x,7,59,61";
        let result = find_earliest_timestamp(input);
        assert_eq!(779210, result);
    }
    #[test]
    fn third_example() {
        let input = "1789,37,47,1889";
        let result = find_earliest_timestamp(input);
        assert_eq!(1202161486, result);
    }
}
