fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|row| row.chars().map(|t| t.to_string()).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    const NEIGHBORS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
    ];

    let mut result = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, _col) in row.iter().enumerate() {
            let mut count = 0;
            if &input[y][x] == "." {
                continue;
            }
            for (ny, nx) in NEIGHBORS.iter() {
                let (new_y, new_x) = (y as i32 + ny, x as i32 + nx);
                if new_y >= 0
                    && (new_y as usize) < input.len()
                    && new_x >= 0
                    && (new_x as usize) < input.len()
                {
                    let node = &input[new_y as usize][new_x as usize];
                    if node == "@" {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                result += 1;
            }
        }
    }

    println!("Part 1: {result}");
    part2();
}

fn part2() {
    let mut input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|row| row.chars().map(|t| t.to_string()).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    const NEIGHBORS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
    ];

    let mut result = 0;
    let mut prev = -1;

    while prev != result {
        if result == prev {
            break;
        }
        prev = result;
        for (y, row) in input.clone().iter().enumerate() {
            for (x, _col) in row.iter().enumerate() {
                let mut count = 0;
                if &input[y][x] == "." {
                    continue;
                }
                for (ny, nx) in NEIGHBORS.iter() {
                    let (new_y, new_x) = (y as i32 + ny, x as i32 + nx);
                    if new_y >= 0
                        && (new_y as usize) < input.len()
                        && new_x >= 0
                        && (new_x as usize) < input.len()
                    {
                        let node = &input[new_y as usize][new_x as usize];
                        if node == "@" {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    input[y][x] = String::from(".");
                    result += 1;
                }
            }
        }
    }

    println!("Part 1: {result}");
}
