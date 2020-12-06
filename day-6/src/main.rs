use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    // let mut group_index = 0;
    let mut group: Vec<&str> = Vec::new();
    let lines_count = lines.clone().count();
    let mut index = 0;
    let groups: Vec<Vec<&str>> = lines.fold(Vec::new(), |mut acc, line| {
        index += 1;

        if index == lines_count {
            group.push(line);
            acc.push(group.clone());
            return acc;
        }

        if line.is_empty() || index == lines_count {
            acc.push(group.clone());
            group = Vec::new();
        } else {
            group.push(line);
        }

        acc
    });

    let groups_sum: Vec<i32> = groups
        .iter()
        .map(|group| {
            let all_responses = group.concat();
            let mut letters: Vec<char> = vec![];
            all_responses.chars().for_each(|c| {
                if !letters.contains(&c) {
                    letters.push(c);
                }
            });
            println!("{}", all_responses);
            println!("{:?}", letters);
            letters.len() as i32
        })
        .collect();

    let result: i32 = groups_sum.iter().sum();

    // println!("{:?}", groups);
    println!("{:?}", groups_sum);
    println!("{:?}", result);
}
