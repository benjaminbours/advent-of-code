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
        let min = min_max[0];
        let max = min_max[1];
        // println!("{:?}", min_max);

        // password.chars().for_each(|x| {
        // });
        let mut count = 0;
        let mut password_chars = password.chars();
        loop {
            let c = password_chars.next();

            if c.is_none() {
                break;
            }

            if c.unwrap().to_string() == letter {
                println!("{}, {}, {}", c.unwrap().to_string(), letter, count);
                count += 1;
            }
        }

        if count <= max && count >= min {
            valid_password_count += 1;
            println!("password: {} is valid", password);
        }
    });

    println!("The number of valid password is {}", valid_password_count);
}
