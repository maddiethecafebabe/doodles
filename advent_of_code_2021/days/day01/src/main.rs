use shared::{Input, Reporter};

pub fn part1(numbers: &Vec<usize>) -> usize {
    numbers
        .as_slice()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

pub fn part2(numbers: &Vec<usize>) -> usize {
    numbers
        .as_slice()
        .windows(3)
        .collect::<Vec<&[usize]>>()
        .windows(2)
        .filter(|&c| c[0].into_iter().sum::<usize>() < c[1].into_iter().sum::<usize>())
        .count()
}

fn main() {
    let nums = Input::from_env_args().unwrap().into_num_vec();

    Reporter::day(1)
        .part1(|| part1(&nums))
        .part2(|| part2(&nums))
        .print()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn part1_test() {
        let nums = Input::from_str(TEST_INPUT);

        assert_eq!(part1(&nums.into_num_vec()), 7);
    }

    #[test]
    fn part2_test() {
        let nums = Input::from_str(TEST_INPUT);

        assert_eq!(part2(&nums.into_num_vec()), 5);
    }
}
