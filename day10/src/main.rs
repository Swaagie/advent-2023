use std::fs;

#[derive(Debug, Clone)]
struct Position<'a> {
    // ugly who cares
    board: &'a Vec<Vec<char>>,
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    West,
    South,
    North,
}

impl<'a> Position<'a> {
    fn east(&self) -> Self {
        Position {
            board: &self.board,
            x: self.x + 1,
            y: self.y,
        }
    }

    fn west(&self) -> Self {
        Position {
            board: &self.board,
            x: self.x - 1,
            y: self.y,
        }
    }

    fn south(&self) -> Self {
        Position {
            board: &self.board,
            x: self.x,
            y: self.y + 1,
        }
    }

    fn north(&self) -> Self {
        Position {
            board: self.board,
            x: self.x,
            y: self.y - 1,
        }
    }

    fn compare(&self, other: &Self) -> Direction {
        if self.x > other.x {
            Direction::West
        } else if self.x < other.x {
            Direction::East
        } else if self.y > other.y {
            Direction::North
        } else if self.y < other.y {
            Direction::South
        } else {
            unreachable!()
        }
    }

    fn pipe(&self) -> char {
        self.board[self.y][self.x]
    }

    fn walk(&self, direction: Direction) -> Self {
        match direction {
            Direction::East => self.east(),
            Direction::West => self.west(),
            Direction::South => self.south(),
            Direction::North => self.north(),
        }
    }
}

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let lines = input.split('\n').collect::<Vec<_>>();

    let lines = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let Some((y, column)) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(&'S'))
    else {
        unreachable!();
    };

    let x = column.iter().position(|&c| c == 'S').unwrap();
    let start = Position {
        board: &lines,
        x,
        y,
    };

    let mut possible = vec![];
    if start.y != 0 {
        possible.push(Position {
            board: &lines,
            x: start.x,
            y: start.y - 1,
        });
    }

    if start.x != 0 {
        possible.push(Position {
            board: &lines,
            x: start.x - 1,
            y: start.y,
        });
    }

    if start.x != lines[0].len() {
        possible.push(Position {
            board: &lines,
            x: start.x + 1,
            y: start.y,
        });
    }

    if start.y != lines.len() {
        possible.push(Position {
            board: &lines,
            x: start.x,
            y: start.y + 1,
        });
    }

    for point in possible {
        let mut steps: i32 = 0;
        let mut previous = start.clone();
        let mut current = point.clone();
        let mut dead_end = false;

        while current.pipe() != 'S' {
            let refer_pos = current.compare(&previous);
            previous = current.clone();

            println!("{:?} ", current.pipe());
            match current.pipe() {
                '|' => {
                    current = match refer_pos {
                        Direction::South => current.walk(Direction::North),
                        Direction::North => current.walk(Direction::South),
                        _ => {
                            dead_end = true;
                            break;
                        }
                    };
                }
                'L' => {
                    current = match refer_pos {
                        Direction::North => current.walk(Direction::East),
                        Direction::East => current.walk(Direction::North),
                        _ => {
                            dead_end = true;
                            break;
                        }
                    };
                }
                '-' => {
                    current = match refer_pos {
                        Direction::East => current.walk(Direction::West),
                        Direction::West => current.walk(Direction::East),
                        _ => {
                            dead_end = true;
                            break;
                        }
                    };
                }
                'J' => {
                    current = match refer_pos {
                        Direction::West => current.walk(Direction::North),
                        Direction::North => current.walk(Direction::West),
                        _ => {
                            dead_end = true;
                            break;
                        }
                    };
                }
                '7' => {
                    current = match refer_pos {
                        Direction::South => current.walk(Direction::West),
                        Direction::West => current.walk(Direction::South),
                        _ => {
                            dead_end = true;
                            break;
                        }
                    };
                }
                'F' => {
                    current = match refer_pos {
                        Direction::South => current.walk(Direction::East),
                        Direction::East => current.walk(Direction::South),
                        _ => {
                            dead_end = true;
                            break;
                        }
                    };
                }
                _ => {
                    dead_end = true;
                    break;
                }
            }

            steps += 1;
            println!("{:?} steps, {:?}", current.pipe(), steps);
        }

        if !dead_end {
            println!("Result {:?}", (steps as f32 / 2.0).ceil());
        }
    }
}
