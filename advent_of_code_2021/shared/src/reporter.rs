use std::fmt;

#[derive(Debug)]
pub struct Reporter {
    day: usize,
    part1: Option<Box<dyn fmt::Debug>>,
    part2: Option<Box<dyn fmt::Debug>>,
}

impl Reporter {
    pub fn day(d: usize) -> Self {
        Self {
            day: d,
            part1: None,
            part2: None,
        }
    }

    pub fn part1<D: fmt::Debug + 'static>(mut self, f: impl FnOnce() -> D) -> Self {
        self.part1 = Some(Box::new(f()));
        self
    }

    pub fn part2<D: fmt::Debug + 'static>(mut self, f: impl FnOnce() -> D) -> Self {
        self.part2 = Some(Box::new(f()));
        self
    }

    pub fn print(self) {
        println!(
            "===== Day {} =====\n Part1: {:?}\n Part2: {:?}\n",
            self.day,
            self.part1.unwrap_or(Box::new("<none>")),
            self.part2.unwrap_or(Box::new("<none>"))
        );
    }
}
