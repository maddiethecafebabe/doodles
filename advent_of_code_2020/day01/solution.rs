use std::env;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum BruteOption {
    Double(u32, u32, u32),
    Triple(u32, u32, u32, u32),
    None,
}

fn read_entries(path: &String) -> io::Result<Vec<u32>> {
    let fp = fs::File::open(&path)?;
    let mut r = vec![];

    for line in io::BufReader::new(fp).lines() {
        r.push(line?.parse().unwrap())
    }
    Ok(r)
}

fn brute(entries: &Vec<u32>, wanted: u32, summand_cnt: u8) -> BruteOption {
    if summand_cnt == 2 {
        for n in entries.iter() {
            for i in entries.iter() {
                if n + i == wanted {
                    return BruteOption::Double(*n, *i, n * i);
                }
            }
        }
    } else if summand_cnt == 3 {
        for n in entries.iter() {
            for i in entries.iter() {
                for a in entries.iter() {
                    if a + n + i == wanted {
                        return BruteOption::Triple(*a, *n, *i, a * n * i);
                    }
                }
            }
        }
    }
    BruteOption::None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} path/to/input.txt", args[0]);
        return;
    }
    let path = &args[1];

    let entries = read_entries(&path).expect("Failed to read entries from file");
    for i in 2..4 {
        println!("{:?}", brute(&entries, 2020, i))
    }
}
