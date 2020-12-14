use itertools::{EitherOrBoth::*, Itertools};
use regex::Regex;
use std::{collections::HashMap, fmt, fs, iter, u64};

const MASK: &str = "mask";

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
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

            let value = right.parse::<u64>().unwrap();
            let modified_mem_address = modify_bits(memory_address, mask);
            write_to_memory(modified_mem_address, value, &mut memory);
        }
    });

    let sum: u64 = memory.iter().map(|x| x.1).sum();

    println!("{:?}", memory);
    println!("{:?}", sum);
}

fn modify_bits(number: u32, mask: &str) -> String {
    let binary_number = format!("{:b}", number);

    let mut rewrite_number_chars: Vec<char> = mask
        .chars()
        .rev()
        .zip_longest(binary_number.chars().rev())
        .fold(vec![], |mut acc, pair| {
            match pair {
                Both(mask_char, number_char) => match mask_char {
                    '0' => acc.push(number_char),
                    '1' => acc.push('1'),
                    'X' => acc.push('X'),
                    _ => (),
                },
                Left(mask_char) => match mask_char {
                    '0' => acc.push(mask_char),
                    '1' => acc.push('1'),
                    'X' => acc.push('X'),
                    _ => (),
                },
                _ => (), // Right(number_char) => acc.push(number_char),
            }
            acc
        });
    rewrite_number_chars.reverse();
    let modified_number: String = rewrite_number_chars.into_iter().collect();
    // modified_number.chars().

    println!("modified: {}", modified_number);

    modified_number
}

fn write_to_memory(address: String, value: u64, memory: &mut HashMap<u64, u64>) {
    let chars_to_replace: Vec<(usize, char)> = address
        .chars()
        .enumerate()
        .filter(|(_, x)| x == &'X')
        .collect();
    let length = chars_to_replace.iter().count() as i32;

    let combination_count = 2i32.pow(length as u32);
    println!("length: {}", length);
    println!("combination count: {}", combination_count);

    let mut index = 0;
    loop {
        if index == combination_count {
            break;
        }

        let number_full: String = {
            let binary_number = format!("{:b}", index);
            // println!("{}", binary_number);
            let rest = length - binary_number.len() as i32;

            if rest != 0 {
                let front: String = iter::repeat('0').take(rest as usize).collect();
                format!("{}{}", front, binary_number)
            } else {
                binary_number
            }
        };
        println!("{}", number_full);

        let mut address_updated = address.clone();
        chars_to_replace.iter().zip(number_full.chars()).for_each(
            |((char_index, _), edited_char)| {
                address_updated.remove(char_index.clone());
                address_updated.insert(char_index.clone(), edited_char);
            },
        );

        println!("{}", address_updated);
        let memory_address = u64::from_str_radix(&address_updated, 2).unwrap();
        println!("{}", memory_address);

        match memory.get_mut(&memory_address) {
            Some(x) => {
                *x = value;
            }
            None => {
                memory.insert(memory_address, value);
            }
        }

        index += 1;
    }
}
