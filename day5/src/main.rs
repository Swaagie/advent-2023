use std::fs;

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut inputs = input.split("\n---\n").collect::<Vec<_>>();
    let seeds = inputs.remove(0).split_whitespace().collect::<Vec<_>>();
    let mut seed_map = vec![];

    // build seed map
    let mut seeds = seeds.iter().peekable();
    while seeds.peek().is_some() {
        let start = seeds.next().unwrap().parse::<i64>().unwrap();
        let length = seeds.next().unwrap().parse::<i64>().unwrap();
        seed_map.push(start..start + length);
    }

    // builds maps
    let mut maps = vec![];
    for map in inputs {
        let lines = map.split("\n").collect::<Vec<_>>();
        let mut map = vec![];

        for line in lines {
            let data = line.split_whitespace().collect::<Vec<_>>();
            let target = data[0].parse::<i64>().unwrap();
            let source = data[1].parse::<i64>().unwrap();
            let length = data[2].parse::<i64>().unwrap();
            map.push((source..source + length, target - source));
        }

        map.sort_by(|a, b| a.0.start.cmp(&b.0.start));
        maps.push(map);
    }

    let mut locations: Vec<i64> = vec![];
    for range in seed_map {
        println!("SEED {:?}", range);
        let mut next = range;
        for map in &maps {
            for lookup in map {
                let start = range.start;
                if start > lookup.0.end {
                    continue;
                } else {
                    if range.end > lookup.0.end {
                        seed_map.push(lookup.0.end..range.end);
                        continue;
                    };
                }

                // println!("map {:?}\nlookup {:?}", map, lookup);
                if lookup.0.contains(&next) {
                    next = next + lookup.1;
                    // println!("next {}", next);
                    break;
                }
            }
        }

        locations.push(next);
    }

    println!("Result: {:?}", locations.iter().min().unwrap());
}
