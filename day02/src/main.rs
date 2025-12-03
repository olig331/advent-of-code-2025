fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", solve(input.as_str(), false));
    println!("Part 2: {}", solve(input.as_str(), true));
}

#[rustfmt::skip]
fn solve(input: &str, part2: bool) -> u64 {
    input.lines().map(|range| {
        let x = range.split('-').collect::<Vec<_>>();
        let (start, end) = (x[0].parse::<usize>().unwrap(), x[1].parse::<usize>().unwrap());

        (start..=end).filter(|&num| {
            let s = num.to_string();
            if !part2 {
                s.len() % 2 == 0 && s[0..s.len()/2] == s[s.len()/2..]
            } else {
                (1..=10).any(|size| s.len() % size == 0 && s.len()/size > 1 &&
                    s.chars()
                        .collect::<Vec<_>>()
                        .chunks(size)
                        .map(|c| c.iter().collect::<String>())
                        .all(|chunk| chunk == s[0..size]))
            }
        }).map(|num| num as u64).sum::<u64>()
    }).sum()
}
