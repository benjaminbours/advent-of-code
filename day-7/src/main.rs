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
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut color_rules: HashMap<String, Option<Vec<Box<Bag>>>> = HashMap::new();
    lines.for_each(|line| {
        let mut parts = line.splitn(2, "bags");
        let color = parts.next().unwrap().trim();

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

                        let bag = Bag {
                            quantity: Some(quantity),
                            color: color.trim().to_owned(),
                        };

                        Box::new(bag)
                    })
                    .collect::<Vec<Box<Bag>>>(),
            );
        }

        color_rules.insert(color.to_owned(), bags);
    });

    // all bags that can be in a shiny gold bag
    let tmp = color_rules.get(MY_BAG_COLOR).unwrap();

    // let result = tmp.and_then(|x| {
    //     // iter into first level of bag
    //     x.iter().map(|bag| {
    //         // bags inside first level bag
    //         let bag_inside = color_rules.get(&bag.color).unwrap();
    //         let inside_count = 3;

    //         inside_count * bag.quantity.unwrap()
    //     });
    //     // x.iter().
    //     // let count = x.iter().fold(0, |acc, bag| {
    //     //     // b.color
    //     //     acc
    //     // });

    //     Some(3)
    // });

    let result = bags_count(tmp, &color_rules);
    // .unwrap()

    println!("{:?}", result);
}

fn part_one() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut color_rules: HashMap<String, Option<Vec<Box<Bag>>>> = HashMap::new();

    lines.for_each(|line| {
        let mut parts = line.splitn(2, "bags");
        let color = parts.next().unwrap().trim();

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

                        let bag = Bag {
                            quantity: Some(quantity),
                            color: color.trim().to_owned(),
                        };

                        Box::new(bag)
                    })
                    .collect::<Vec<Box<Bag>>>(),
            );
        }

        color_rules.insert(color.to_owned(), bags);
    });

    let result: Vec<&String> = color_rules
        .iter()
        .filter(|(key, value)| {
            println!("{:?}", key);
            can_contains_shiny(value, &color_rules)
            // false
        })
        .map(|(key, value)| key)
        .collect();

    println!("{:?}", result.len());
}

fn can_contains_shiny(
    bags: &Option<Vec<Box<Bag>>>,
    all_bags: &HashMap<String, Option<Vec<Box<Bag>>>>,
) -> bool {
    match bags {
        Some(bgs) => bgs.iter().any(|b| {
            let tmp = all_bags.get(&b.color).unwrap();
            let condition = b.color == MY_BAG_COLOR && b.quantity.unwrap() >= 1;
            condition || can_contains_shiny(tmp, all_bags)
        }),
        None => false,
    }
}

fn bags_count(
    bags: &Option<Vec<Box<Bag>>>,
    all_bags: &HashMap<String, Option<Vec<Box<Bag>>>>,
    // count: Option<usize>,
) -> usize {
    // let start = match count {
    //     Some(c) => c,
    //     None => 0,
    // };

    match bags {
        Some(bgs) => {
            bgs.iter().map(|x| {
                let quantity = x.quantity.unwrap();
                // println!("bag: {:?}", x);
                // println!("acc: {}", acc);
                let bag_inside = all_bags.get(&x.color).unwrap();

                quantity + quantity * bags_count(bag_inside, all_bags)

                // x.quantity.unwrap() + x.quantity.unwrap() *
                // x.quantity.unwrap() * bags_count(&bag_inside, all_bags, Some(acc));
                // acc
            }).sum()
            // 1
        }
        None => 0,
    }
}
