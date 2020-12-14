use itertools::{EitherOrBoth::*, Itertools};
use regex::Regex;
use std::{collections::HashMap, fmt, fs, u64};

const MASK: &str = "mask";
const DONT_EDIT: char = 'X';

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut memory: HashMap<u32, u64> = HashMap::new();
    let mut mask = "0";

    input.lines().for_each(|line| {
        let mut parts = line.split('=').map(|x| x.trim());
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        if left == MASK {
            mask = right;
        } else {
            let re = Regex::new(r"^mem\[(.*)\]$").unwrap();
            println!("{}", left);
            let memory_address = re
                .captures(left)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            println!("{:?}", memory_address);

            let binary_right = format!("{:b}", right.parse::<u64>().unwrap());

            println!("binary of {}: {:?}", right, binary_right);
            let mut rewrite_number_chars: Vec<char> = mask
                .chars()
                .rev()
                .zip_longest(binary_right.chars().rev())
                .fold(vec![], |mut acc, pair| {
                    match pair {
                        Both(mask_char, number_char) => {
                            if mask_char == DONT_EDIT {
                                acc.push(number_char)
                            } else {
                                acc.push(mask_char)
                            }
                        }
                        Left(mask_char) => {
                            if mask_char == DONT_EDIT {
                                acc.push('0');
                            } else {
                                acc.push(mask_char)
                            }
                        }
                        Right(number_char) => acc.push(number_char),
                    }
                    acc
                });
            rewrite_number_chars.reverse();
            let edited_number: String = rewrite_number_chars.into_iter().collect();
            let value_written_in_mem = u64::from_str_radix(&edited_number, 2).unwrap();

            match memory.get_mut(&memory_address) {
                Some(x) => {
                    *x = value_written_in_mem;
                },
                None => {
                    memory.insert(memory_address, value_written_in_mem);
                },
            }
            // println!(
            //     "edited binary: {}, edited value: {:?}",
            //     edited_number,
            //     u64::from_str_radix(&edited_number, 2).unwrap()
            // );
        }
    });

    let sum: u64 = memory.iter().map(|x| x.1).sum();

    println!("{:?}", memory);
    println!("{:?}", sum);
}
