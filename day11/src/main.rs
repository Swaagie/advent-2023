use std::fs;

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let lines = input.split('\n').collect::<Vec<_>>();

    let galaxy = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut positions = vec![];
    for row in galaxy.iter().enumerate() {
        row.1
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == '#')
            .for_each(|(i, _)| positions.push(Position { x: i, y: row.0 }));
    }

    let mut result: i64 = 0;
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let mut columns_without_galaxy = 0;
            let start = positions[i].x.min(positions[j].x);
            let end = positions[i].x.max(positions[j].x);

            // Not performant doing this every time, but it works
            for k in start..end {
                let mut has_galaxy = false;

                for row in &galaxy {
                    if row[k] == '#' {
                        has_galaxy = true;
                        break;
                    }
                }

                if !has_galaxy {
                    columns_without_galaxy += 1;
                }
            }

            // Same brute force iteration
            let rows_without_galaxy = galaxy[positions[i].y..positions[j].y]
                .iter()
                .filter(|line| !line.contains(&'#'))
                .count() as i64;

            let distance = (positions[i].x as i64 - positions[j].x as i64).abs()
                + (columns_without_galaxy * (1000000 - 1))
                + (positions[i].y as i64 - positions[j].y as i64).abs()
                + (rows_without_galaxy * (1000000 - 1));

            println!(
                "{:?} {:?}, dist: {:?}",
                positions[i], positions[j], distance
            );
            result += distance;
        }
    }

    print!("{}", result);
}
