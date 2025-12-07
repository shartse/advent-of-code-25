use std::{
    collections::HashMap, fs::File, io::{self, BufRead}
};

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Cell>>,
    dimensions: (usize, usize),
    start: (usize, usize),
}

#[derive(Debug, PartialEq)]
enum Cell {
    Start,
    Splitter,
    Beam,
    Empty,
}

impl Grid {
    fn new(lines: Vec<String>) -> Self {
        let rows: Vec<Vec<Cell>> = lines
            .into_iter()
            .map(|row| {
                row.chars()
                    .map(|c| {
                        if c == 'S' {
                            Cell::Start
                        } else if c == '^' {
                            Cell::Splitter
                        } else {
                            Cell::Empty
                        }
                    })
                    .collect()
            })
            .collect();
        let dimensions = (rows.len(), rows[0].len());
        let start_x = rows[0].iter().position(|x| x == &Cell::Start).unwrap();
        Grid {
            rows,
            dimensions,
            start: (0, start_x),
        }
    }

    fn pretty_print(&self) {
        for r in &self.rows {
            println!("");
            for c in r.into_iter() {
                let s = match c {
                    Cell::Start => "S",
                    Cell::Splitter => "^",
                    Cell::Beam => "|",
                    Cell::Empty => ".",
                };
                print!("{}", s)
            }
        }
    }

    fn is_split(&self, pos: (usize, usize)) -> bool {
        match self.rows[pos.0][pos.1] {
            Cell::Start => panic!("should not be stepping to start"),
            Cell::Splitter => true,
            Cell::Beam => false,
            Cell::Empty => false,
        }
    }

    fn num_paths(&self, paths: usize, pos: (usize, usize), memo: &mut HashMap<(usize,usize),usize>) -> usize {
        if let Some(num) = memo.get(&pos) {
            return *num
        }
        if pos.0 == self.dimensions.0 - 1 {
            paths
        } else {
           let result =  match self.rows[pos.0 + 1][pos.1] {
                Cell::Start => {
                    panic!("we shouldn't be checking the start")
                }
                Cell::Empty => self.num_paths(paths, (pos.0 + 1, pos.1), memo),
                Cell::Splitter => {
                    if pos.1 < self.dimensions.1 - 1 && pos.1 > 0 {
                        self.num_paths(paths, (pos.0 + 1, pos.1 - 1), memo)
                            + self.num_paths(paths, (pos.0 + 1, pos.1 + 1), memo)
                    } else {
                        if pos.1 < self.dimensions.1 - 1 {
                            self.num_paths(paths, (pos.0 + 1, pos.1 + 1),memo)
                        } else if pos.1 > 0 {
                            self.num_paths(paths, (pos.0 + 1, pos.1 - 1),memo)
                        } else {
                            0
                        }
                    }
                }
                Cell::Beam => panic!("should not be hitting a beam"),
            };
            memo.insert(pos, result);
            result
        }
    }

    fn all_possible_routes(&mut self) -> usize {
        let mut memo = HashMap::new();
        self.num_paths(1, self.start, &mut memo)
    }

    fn run_grid(&mut self) -> usize {
        let mut split_count = 0;
        for r in 0..self.dimensions.0 - 1 {
            for c in 0..self.dimensions.1 {
                match self.rows[r][c] {
                    Cell::Start | Cell::Beam => {
                        let next = (r + 1, c);
                        if self.is_split(next) {
                            if next.1 < self.dimensions.1 - 1 {
                                self.rows[next.0][next.1 + 1] = Cell::Beam;
                            }
                            if next.1 > 0 {
                                self.rows[next.0][next.1 - 1] = Cell::Beam;
                            }
                            split_count += 1;
                        } else {
                            self.rows[next.0][next.1] = Cell::Beam;
                        }
                    }
                    Cell::Empty | Cell::Splitter => continue,
                }
            }
        }
        split_count
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    part_1(values.clone());
    part_2(values);
}

fn part_1(vals: Vec<String>) {
    let mut grid = Grid::new(vals);
    let splits = grid.run_grid();
    grid.pretty_print();
    println!("Total number of splits is: {splits}");
}

fn part_2(vals: Vec<String>) {
    let mut grid = Grid::new(vals);
    let routes = grid.all_possible_routes();
    println!("Total number of paths is: {routes}");
}
