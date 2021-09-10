use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    iter::FromIterator,
};

fn bitconv(s: &str) -> u32 {
    u32::from_str_radix(
        &s.replace("B", "1")
            .replace("F", "0")
            .replace("R", "1")
            .replace("L", "0"),
        2,
    )
    .unwrap()
}

fn read_passport_ids(path: &String) -> io::Result<Vec<u32>> {
    let mut map = Vec::from_iter(
        BufReader::new(File::open(path)?)
            .lines()
            .map(|l| -> (String, String) {
                let l = l.unwrap();
                let (r, s) = l.split_at(7);
                (r.to_string(), s.to_string())
            })
            .map(|(r, s)| bitconv(&r) * 8 + bitconv(&s)),
    );
    map.sort();
    Ok(map)
}

fn main() {
    let fpath: String = env::args().nth(1).unwrap_or("input.txt".to_string());

    let ids = read_passport_ids(&fpath).unwrap();
    println!("Day5\n  Part1: {}", ids.last().unwrap());

    let my_seat = ids
        .iter()
        .filter(|id| {
            !ids.contains(&(*id + 1)) && Some(*id) != ids.last() && Some(*id) != ids.first()
        })
        .nth(0)
        .unwrap()
        + 1;
    println!("  Part2: {}", my_seat);
}
