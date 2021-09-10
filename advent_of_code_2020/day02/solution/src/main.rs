use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, BufRead};

struct Entry {
    lower: usize,
    upper: usize,
    pw_char: char,
    password: String,
}

fn read_entries(path: &String) -> io::Result<Vec<Entry>> {
    let fp = fs::File::open(path)?;
    let mut r = vec![];
    let re = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();

    for line in io::BufReader::new(fp).lines() {
        for mgroup in re.captures_iter(&line?) {
            r.push(Entry {
                lower: mgroup[1].parse().unwrap(),
                upper: mgroup[2].parse().unwrap(),
                pw_char: mgroup[3].to_string().chars().nth(0).unwrap(),
                password: mgroup[4].to_string(),
            })
        }
    }

    Ok(r)
}

fn old_is_valid_password(entry: &Entry) -> bool {
    let char_cnt = entry.password.matches(entry.pw_char).count();
    entry.lower <= char_cnt && char_cnt <= entry.upper
}

fn new_is_valid_password(entry: &Entry) -> bool {
    let first = entry
        .password
        .chars()
        .nth((entry.lower - 1).into())
        .unwrap();
    let second = entry
        .password
        .chars()
        .nth((entry.upper - 1).into())
        .unwrap();

    (first == entry.pw_char) ^ (second == entry.pw_char)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fpath = &String::from("./input.txt");
    if args.len() >= 2 {
        fpath = &args[1];
    }

    let mut old_valid_cnt = 0;
    let mut new_valid_cnt = 0;
    for entry in read_entries(fpath).unwrap() {
        old_valid_cnt += old_is_valid_password(&entry) as u32;
        new_valid_cnt += new_is_valid_password(&entry) as u32;
    }

    println!(
        "Valid passwords:\n  old policy: {}\n  new policy: {}",
        old_valid_cnt, new_valid_cnt
    );
}
