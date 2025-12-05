fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (r, v) = input.split_once("\n\n").unwrap();

    let mut ranges = r
        .lines()
        .map(|l| {
            l.split_once("-")
                .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let p1 = v
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|v| ranges.iter().any(|(start, end)| v >= start && v <= end))
        .count() as u64;

    let mut merged: Vec<(u64, u64)> = Vec::new();
    ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (start, end) in ranges {
        if let Some((_, prev_end)) = merged.last_mut()
            && start <= *prev_end + 1
        {
            *prev_end = (*prev_end).max(end);
            continue;
        }
        merged.push((start, end));
    }

    let p2 = merged
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
