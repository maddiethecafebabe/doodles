use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

struct Map {
    inner: Vec<String>,
}

impl Map {
    fn init(path: impl AsRef<Path>) -> Map {
        let fp = File::open(path).expect("Couldn't open file");
        let reader = BufReader::new(fp);

        Map {
            inner: reader.lines().map(|line| line.unwrap()).collect(),
        }
    }

    fn get(&self, x_pos: usize, y_pos: usize) -> char {
        self.inner[y_pos]
            .chars()
            .nth(x_pos % self.inner[0].len())
            .unwrap()
    }

    fn is_tree(&self, x_pos: usize, y_pos: usize) -> bool {
        self.get(x_pos, y_pos) == '#'
    }

    fn traverse_vector(&self, start_pos: (usize, usize), vector: (usize, usize)) -> usize {
        let (mut x_pos, mut y_pos) = start_pos;
        let (x_vec, y_vec) = vector;
        let mut tree_cnt = 0;
        while y_pos < self.inner.len() {
            tree_cnt += self.is_tree(x_pos, y_pos) as usize;
            x_pos += x_vec;
            y_pos += y_vec;
        }
        tree_cnt
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fpath = &String::from("input.txt");
    if args.len() > 1 {
        fpath = &args[1];
    }

    let map = Map::init(fpath);
    println!("Part1: {}", map.traverse_vector((0, 0), (3, 1)));

    let mut prod = 1;
    for v in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        prod *= map.traverse_vector((0, 0), *v);
    }
    println!("Part2: {}", prod);
}
