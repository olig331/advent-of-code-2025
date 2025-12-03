fn main() {
    let (mut start, mut p1_count, mut p2_count) = (50u32, 0, 0);
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            let v = l[1..].parse::<usize>().unwrap();
            p2_count += count_zeros(if l.starts_with('L') { -1 } else { 1 }, v, &mut start);
            if start == 0 {
                p1_count += 1;
            }
        });
    println!("Part 1: {p1_count}");
    println!("Part 2: {p2_count}");
}

fn count_zeros(direction: i8, iter_count: usize, start: &mut u32) -> usize {
    (0..iter_count)
        .filter(|_| {
            *start = ((*start as i8 + direction).rem_euclid(100)) as u32;
            *start == 0
        })
        .count()
}
