use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut valid_password_count = 0;
    input.lines().for_each(|line| {
        let mut temp = line.split(":");
        let policy = temp.next().unwrap().trim();
        let password = temp.next().unwrap().trim();
        println!("policy: {}, password: {}", policy, password);

        // policy
        let mut policy_iterator = policy.split(" ");
        let time = policy_iterator.next().unwrap();
        let letter = policy_iterator.next().unwrap();

        let min_max: Vec<u32> = time.split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let pos1 = min_max[0];
        let pos2 = min_max[1];

        let mut count = 0;
        let mut password_chars = password.chars();
        let mut iterator_index = 0;
        loop {
            iterator_index += 1;
            let c = password_chars.next();

            if c.is_none() {
                break;
            }

            if (iterator_index == pos1 || iterator_index == pos2)
                && c.unwrap().to_string() == letter
            {
                println!("{}, {}, {}", c.unwrap().to_string(), letter, iterator_index);
                count += 1;
            }
        }
        println!("count: {}", count);
        if count == 1 {
            valid_password_count += 1;
            println!("password: {} is valid", password);
        }
    });

    println!("The number of valid password is {}", valid_password_count);
}
