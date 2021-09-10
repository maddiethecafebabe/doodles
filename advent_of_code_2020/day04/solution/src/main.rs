use regex::Regex;
use std::{collections::HashMap, env, fs::read_to_string, io};

struct Passport {
    map: HashMap<String, String>,
}

impl Passport {
    fn from_string(string: String, pattern: &Regex) -> Passport {
        let map: HashMap<String, String> = pattern
            .captures_iter(&string)
            .map(|m| (m[1].to_string(), m[2].to_string()))
            .collect();
        Passport { map }
    }

    fn has_all_needed(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .filter(|k| !self.map.contains_key(&k.to_string()))
            .count()
            == 0
    }

    fn is_valid(&self) -> bool {
        self.has_all_needed()
            && self
                .map
                .iter()
                .filter(|(k, v)| match k.as_str() {
                    "byr" => Passport::valid_int_range(v, 1920, 2002, 4, 10),
                    "iyr" => Passport::valid_int_range(v, 2010, 2020, 4, 10),
                    "eyr" => Passport::valid_int_range(v, 2020, 2030, 4, 10),
                    "hgt" => match (
                        &v[..v.len() - 2].parse::<u32>().unwrap_or(0),
                        &v[v.len() - 2..],
                    ) {
                        (i, "cm") => 150 <= *i && *i <= 193,
                        (i, "in") => 59 <= *i && *i <= 76,
                        _ => false,
                    },
                    "hcl" => {
                        v.starts_with("#")
                            && Passport::valid_int_range(
                                &v.trim_start_matches("#").to_string(),
                                -1,
                                -1,
                                6,
                                16,
                            )
                    }
                    "ecl" => {
                        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                            .iter()
                            .filter(|x| x == v)
                            .count()
                            != 0
                    }
                    "pid" => Passport::valid_int_range(v, -1, -1, 9, 10),
                    "cid" => true,
                    _ => {
                        println!("wut: {:?}", k);
                        false
                    }
                })
                .count()
                == self.map.len()
    }

    fn valid_int_range(s: &String, low: i32, up: i32, len: usize, base: u32) -> bool {
        match u128::from_str_radix(s, base) {
            Ok(_) => {
                if low == -1 || up == -1 {
                    s.len() == len
                } else {
                    let i = s.parse::<i32>().unwrap();
                    s.len() == len && low <= i && i <= up
                }
            }
            Err(_) => false,
        }
    }

    fn load_passports(fpath: String) -> io::Result<Vec<Passport>> {
        let reg = Regex::new(r"(\w+)(?::)(\S+)(?:\s|\n)?").unwrap();
        let blocks: Vec<Passport> = read_to_string(fpath)?
            .split("\n\n")
            .map(|s| s.to_string())
            .map(|s| Passport::from_string(s, &reg))
            .collect();
        Ok(blocks)
    }
}

fn main() {
    let fpath = env::args().nth(1).unwrap_or("input.txt".to_string());
    let passports = Passport::load_passports(fpath).unwrap();

    let valid1_cnt = passports.iter().filter(|p| p.has_all_needed()).count();
    let valid2_cnt = passports.iter().filter(|p| p.is_valid()).count();
    println!("Day4:\n  Part1: {}\n  Part2: {}", valid1_cnt, valid2_cnt);
}
