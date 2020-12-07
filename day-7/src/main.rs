use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Bag {
    color: String,
    quantity: Option<usize>,
}

const NO_OTHER_BAGS: &str = "no other bags";
const MY_BAG_COLOR: &str = "shiny gold";

fn main() {
    // TODO: make it recursive
    // TODO: make a dictionary of color
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut color_rules: HashMap<String, Option<Vec<Box<Bag>>>> = HashMap::new();
    // color_rules.entry(key);

    lines.for_each(|line| {
        let mut parts = line.splitn(2, "bags");
        let color = parts.next().unwrap().trim();
        // println!("color: {}", color.trim());

        let rest = parts
            .next()
            .unwrap()
            .replace("contain", "")
            .replace(".", "");

        let mut bags: Option<Vec<Box<Bag>>> = None;
        if rest.trim() != NO_OTHER_BAGS {
            bags = Some(
                rest.split(",")
                    .map(|b| {
                        let mut parts = b.trim().splitn(2, " ");
                        let quantity = parts.next().unwrap().parse::<usize>().unwrap();
                        let mut color = parts.next().unwrap().to_owned();
                        if quantity > 1 {
                            color = color.replace("bags", "");
                        } else {
                            color = color.replace("bag", "");
                        }
                        // let color = parts.next().unwrap().replace("bag", "").replace("bags", "");
                        // println!("quantity: {}, color: {}", quantity, color);

                        let bag = Bag {
                            quantity: Some(quantity),
                            color: color.trim().to_owned(),
                            // bags: None,
                        };

                        // println!("bag: {:?}", bag);
                        Box::new(bag)
                    })
                    .collect::<Vec<Box<Bag>>>(),
            );
        }

        color_rules.insert(color.to_owned(), bags);
    });

    // println!("{:?}", color_rules);

    let result: Vec<&String> = color_rules
        .iter()
        .filter(|(key, value)| {
            println!("{:?}", key);
            can_contains_shiny(value, &color_rules)
            // false
        })
        .map(|(key, value)| key)
        .collect();

    // let result: Vec<Bag> = all_bags
    // .filter(|bag| match &bag.bags {
    //     Some(bags) => bags
    //         .iter()
    //         .any(|b| b.color == MY_BAG_COLOR && b.quantity.unwrap() >= 1),
    //     None => false,
    //     })
    //     .map(|b| {
    //         println!("{:?}", b);
    //         b
    //     })
    //     .collect();

    println!("{:?}", result.len());
}

fn can_contains_shiny(
    bags: &Option<Vec<Box<Bag>>>,
    all_bags: &HashMap<String, Option<Vec<Box<Bag>>>>,
) -> bool {
    match bags {
        Some(bgs) => {
            bgs.iter().any(|b| {
                let tmp = all_bags.get(&b.color).unwrap();
                let condition = b.color == MY_BAG_COLOR && b.quantity.unwrap() >= 1;

                condition || can_contains_shiny(tmp, all_bags)
                // .unwrap()
                // .and_then(|x| can_contains_shiny(x, all_bags))
                // true
            })
        }
        None => false,
    }
}
