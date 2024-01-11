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

    let mut result = vec![];
    for line in lines {
        let pieces = line.split_whitespace().collect::<Vec<_>>();
        let options = pieces[0].chars().collect::<Vec<_>>();
        let nums = pieces[1]
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for n in nums {
            let mut k = 0;

            while options[n + k] == 'x' {
                k += 1;
            }

            println!("{:?}", n);
        }
    }

    print!("{}", result);
}
