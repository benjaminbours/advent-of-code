use regex::Regex;
use std::fs;

#[derive(Default, Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn new(data: Vec<&str>) -> Result<Passport, &str> {
        if !is_passport_containing_require_fields(&data.concat()) {
            return Err("Failed");
        }

        let passport = data.iter().fold(Passport::default(), |mut acc, line| {
            line.split(" ")
                .map(|x| x.split(":").collect())
                .for_each(|x: Vec<&str>| match x[0] {
                    "byr" => acc.byr = x[1].to_string(),
                    "iyr" => acc.iyr = x[1].to_string(),
                    "eyr" => acc.eyr = x[1].to_string(),
                    "hgt" => acc.hgt = x[1].to_string(),
                    "hcl" => acc.hcl = x[1].to_string(),
                    "ecl" => acc.ecl = x[1].to_string(),
                    "pid" => acc.pid = x[1].to_string(),
                    "cid" => acc.cid = Some(x[1].to_string()),
                    _ => (),
                });
            acc
        });

        Ok(passport)
    }

    fn is_byr_valid(&self) -> bool {
        let value_as_string = &self.byr;
        let value_as_int = value_as_string.parse::<i32>().unwrap();
        value_as_string.chars().count() == 4 && value_as_int >= 1920 && value_as_int <= 2002
    }

    fn is_iyr_valid(&self) -> bool {
        let value_as_string = &self.iyr;
        let value_as_int = value_as_string.parse::<i32>().unwrap();
        value_as_string.chars().count() == 4 && value_as_int >= 2010 && value_as_int <= 2020
    }

    fn is_eyr_valid(&self) -> bool {
        let value_as_string = &self.eyr;
        let value_as_int = value_as_string.parse::<i32>().unwrap();
        // let re = Regex::new(r"^(in|cm)$").unwrap();
        value_as_string.chars().count() == 4 && value_as_int >= 2020 && value_as_int <= 2030
    }

    fn is_hgt_valid(&self) -> bool {
        if self.hgt.contains("cm") {
            let end = self.hgt.find("c").unwrap();
            let value = &self.hgt[..end];
            let value_as_number = value.parse::<i32>().unwrap();
            return value_as_number >= 150 && value_as_number <= 193;
        }
        if self.hgt.contains("in") {
            let end = self.hgt.find("i").unwrap();
            let value = &self.hgt[..end];
            let value_as_number = value.parse::<i32>().unwrap();
            return value_as_number >= 59 && value_as_number <= 76;
        }
        false
    }

    fn is_hcl_valid(&self) -> bool {
        let value_as_string = &self.hcl;
        let re = Regex::new(r"^#(?:[0-9a-f]){6}$").unwrap();
        re.is_match(&value_as_string) && value_as_string.chars().count() == 7
    }

    fn is_ecl_valid(&self) -> bool {
        let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valid_colors.iter().any(|x| x == &self.ecl)
    }

    fn is_pid_valid(&self) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();
        re.is_match(&self.pid)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lines = input.lines();
    let mut passport_data = vec![];
    let lines_count = lines.clone().count();
    let mut index = 0;
    let mut valid_count = 0;
    loop {
        let line = lines.next();

        match line {
            Some(line) => {
                index += 1;

                if !line.trim().is_empty() {
                    passport_data.push(line);
                }
                if line.trim().is_empty() || index == lines_count {
                    let passport = Passport::new(passport_data);
                    match passport {
                        Ok(p) => {
                            let valid = p.is_byr_valid()
                                && p.is_iyr_valid()
                                && p.is_eyr_valid()
                                && p.is_hgt_valid()
                                && p.is_hcl_valid()
                                && p.is_ecl_valid()
                                && p.is_pid_valid();

                            if valid {
                                valid_count += 1;
                            }
                        }
                        _ => (),
                    }
                    passport_data = vec![];
                    continue;
                }
            }
            None => {
                break;
            }
        }
    }
    println!("{}", valid_count);
}

fn is_passport_containing_require_fields(data: &String) -> bool {
    let required_fiels = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    required_fiels.iter().all(|required_field| {
        if data.contains(required_field) {
            return true;
        }
        false
    })
}
