use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut one_jolt_diff = 0;
    let mut two_jolt_diff = 0;
    let mut three_jolt_diff = 1;

    // start
    // let charging_outlet = 0;

    // let lower_charger = lines.iter().min().unwrap();

    // let start_index = lines.iter().position(|x| x == lower_charger).unwrap();

    // println!("{}", lower_charger);

    let mut count = 0;
    loop {
        // Stop when no element in the vector
        if lines.len() == 0 {
            break;
        }

        // println!("lines length {}", lines.len());

        let (diff, charger) = find_closer_charger(count, &mut lines);
        count = charger;
        match diff {
            1 => one_jolt_diff += 1,
            // 2 => two_jolt_diff += 1,
            3 => three_jolt_diff += 1,
            _ => (),
        }
        // break;
    }

    println!("one dif: {}, three dif: {}", one_jolt_diff, three_jolt_diff);

    println!("result: {}", one_jolt_diff * three_jolt_diff);

    // lines.for_each(|line| {
    //     let number = line.parse::<i32>().unwrap();
    // });
}

fn find_closer_charger(current_count: i32, lines: &mut Vec<i32>) -> (i32, i32) {
    let lines_cloned = lines.clone();
    let chargers = lines_cloned.iter().filter(|x| {
        // println!("{}, {}", x, current_count + 3);
        if x <= &&(current_count + 3) {
            return true;
        }
        false
    });
    // println!("{:?}", chargers.clone().collect::<Vec<&i32>>());
    
    let charger = chargers.min().unwrap();
    println!("count: {} value found {:?}", current_count, charger);
    let index = lines.iter().position(|x| x == charger).unwrap();
    // println!("index {:?}", index);
    let difference = charger - current_count;
    println!("difference {:?}", difference);
    // remove from lines vec
    lines.remove(index);

    (difference, charger.to_owned())
}
