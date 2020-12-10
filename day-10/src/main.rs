use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let initial_lines: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();

    // let mut arrangements: Vec<Vec<i32>> = vec![];
    // let mut arrangement: Vec<i32> = vec![];
    // let mut count = 0;
    // let mut lines = initial_lines.clone();
    // loop {
    //     // Stop when no element in the vector
    //     if lines.len() == 0 {
    //         break;
    //     }

    //     // println!("lines length {}", lines.len());

    //     let lines_cloned = lines.clone();
    //     let chargers = lines_cloned
    //         .iter()
    //         .filter(|x| {
    //             // println!("{}, {}", x, current_count + 3);
    //             is_number_in_range(x, count, 3)
    //         })
    //         .collect::<Vec<&i32>>();
    //     println!("chargers: {:?}", chargers);
    //     if chargers.len() == 1 {
    //         arrangements.iter_mut().for_each(|a| {
    //             let number = chargers.get(0).unwrap().clone().clone();
    //             a.push(number);
    //         });
    //     }
    //     let charger = chargers.iter().min().unwrap();
    //     // println!("count: {} value found {:?}", current_count, charger);
    //     let index = lines.iter().position(|x| &x == charger).unwrap();
    //     // println!("index {:?}", index);
    //     // let difference = charger - current_count;
    //     // println!("difference {:?}", difference);
    //     count = charger.clone().clone();
    //     lines.remove(index.to_owned());

    //     arrangement.push(charger.clone().clone());
    // }

    // let mut index = 0;
    // let mut count = 0;
    // let mut possibilites_count = 0;
    // arrangement.iter().fold(arrangements, |acc, n| {
    //     // if
    //     // arrangement.get()
    //     let chargers = find_chargers_in_range(count, &arrangement);
    //     println!("{:?}", chargers);

    //     possibilites_count += chargers.len();

    //     chargers.iter().for_each(|x| {
    //         // if acc
    //     });

    //     // index += 1;
    //     // count = n;

    //     acc
    // });

    // // println!("result: {}", one_jolt_diff * three_jolt_diff);
    // println!("single arr: {:?}", arrangement);
    // println!("all arr: {:?}", arrangements);
    // let mut cache: HashMap<i64, i64> = HashMap::new();
    let mut nums = to_numbers(&input).unwrap();
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums[nums.len() - 1] + 3);
    let result = recursive_answer(nums, &mut HashMap::new());

    println!("result {}", result);
}

// fn is_number_in_range(number: &i32, current_count: i32, range: i32) -> bool {
//     number <= &(current_count + range)
// }

// fn find_closer_charger(current_count: i32, lines: &mut Vec<i32>) -> (i32, i32) {
//     let lines_cloned = lines.clone();
//     let chargers = lines_cloned.iter().filter(|x| {
//         // println!("{}, {}", x, current_count + 3);
//         is_number_in_range(x, current_count, 3)
//     });
//     println!("{:?}", chargers.clone().collect::<Vec<&i32>>());

//     let charger = chargers.min().unwrap();
//     // println!("count: {} value found {:?}", current_count, charger);
//     let index = lines.iter().position(|x| x == charger).unwrap();
//     // println!("index {:?}", index);
//     let difference = charger - current_count;
//     // println!("difference {:?}", difference);
//     // remove from lines vec
//     lines.remove(index);

//     (difference, charger.to_owned())
// }

// fn find_chargers_in_range(current_count: i32, lines: &Vec<i32>) -> Vec<&i32> {
//     // let lines_cloned = lines.clone();
//     let chargers = lines
//         .iter()
//         .filter(|x| {
//             // println!("{}, {}", x, current_count + 3);
//             is_number_in_range(x, current_count, 3)
//         })
//         .collect::<Vec<&i32>>();

//     chargers.to_owned()

//     // println!("{:?}", chargers.clone().collect::<Vec<&i32>>());
// }

// from https://github.com/felixge/advent-2020/blob/main/day10-2/src/main.rs I couldn't find the answer myself :(
fn recursive_answer(nums: Vec<i64>, cache: &mut HashMap<i64, i64>) -> i64 {
    if nums.len() <= 2 {
        return 1;
    }

    let mut variants = 0;
    for (i, num) in nums[1..].iter().enumerate() {
        println!("here: {}, {:?}", num, nums);
        if num - nums[0] > 3 {
            break;
        }

        variants += match cache.get(num) {
            Some(cached) => *cached,
            None => {
                let computed = recursive_answer(nums[i + 1..].to_vec(), cache);
                cache.insert(*num, computed);
                computed
            }
        }
    }

    variants
}

fn to_numbers(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    s.trim().split("\n").map(|x| x.parse()).collect()
}