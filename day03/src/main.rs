fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (mut first, mut second) = (0, 0);
            let nums = l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            for d in nums.iter().take(nums.len() - 1) {
                if *d > first {
                    first = *d;
                    second = 0;
                } else if *d > second {
                    second = *d;
                }
            }
            if *nums.last().unwrap() > second {
                second = *nums.last().unwrap();
            }
            format!("{first}{second}").parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut result = vec![];
            let nums = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

            let (mut max_start, mut max_end) = (0usize, nums.len() - 11);
            while result.len() != 12 {
                let mut highest: u32 = 0;
                let mut highest_idx = 0;
                for (idx, num) in nums[max_start..max_end].iter().enumerate() {
                    if *num > highest {
                        highest = *num;
                        highest_idx = idx;
                    }
                }

                result.push(highest.to_string());
                max_end += 1;
                max_start += highest_idx + 1;
            }

            result.join("").parse::<u64>().unwrap()
        })
        .sum()
}
