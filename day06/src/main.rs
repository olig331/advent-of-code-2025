fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let whitespace_indexes: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| {
            l.char_indices()
                .filter(|&(_, c)| c.is_whitespace())
                .map(|(i, _)| i)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let breakpoints: Vec<usize> = whitespace_indexes
        .first()
        .unwrap()
        .iter()
        .cloned()
        .filter(|idx| whitespace_indexes.iter().all(|v| v.contains(idx)))
        .collect();

    let mut columns: Vec<Vec<&str>> = vec![Vec::new(); breakpoints.len() + 1];
    for line in &lines {
        let mut prev = 0;
        for (i, &bp) in breakpoints.iter().enumerate() {
            columns[i].push(&line[prev..bp]);
            prev = bp + 1;
        }
        columns[breakpoints.len()].push(&line[prev..]);
    }

    let p1 = columns
        .clone()
        .iter_mut()
        .map(|col| match col.pop().unwrap().trim() {
            "*" => col
                .iter()
                .map(|c| c.trim().parse::<u64>().unwrap())
                .product::<u64>(),
            "+" => col
                .iter()
                .map(|c| c.trim().parse::<u64>().unwrap())
                .sum::<u64>(),
            _ => 0u64,
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<u64>();

    let p2 = columns
        .iter_mut()
        .map(|col| {
            let len = col[0].len();
            let op = col.pop().unwrap().trim();
            let vals = (0..len).map(|idx| {
                col.iter()
                    .map(|c| c.chars().nth(idx).unwrap())
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            });
            match op {
                "*" => vals.product::<u64>(),
                "+" => vals.sum::<u64>(),
                _ => 0u64,
            }
        })
        .sum::<u64>();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
