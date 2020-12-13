use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let earliest_timestamp = lines.next().unwrap().parse::<i32>().unwrap();
    let buses: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    println!("{:?}", buses);

    let mut index = earliest_timestamp;

    loop {
        let departure = buses.iter().find(|bus_id| {
            let rest = index % bus_id.to_owned();
            println!("{:?}", rest);
            rest == 0
        });

        match departure {
            Some(d) => {
                let time_waiting = index - earliest_timestamp;
                println!("first departure: {}, timestamp: {}", d, index);
                println!("time waiting: {}", time_waiting);
                println!("res: {}", time_waiting * d);
                break;
            }
            None => index += 1,
        }

        // index += 1;
    }
}
