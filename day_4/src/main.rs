use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<bool>>,
    dimensions: (usize, usize),
}

impl Grid {
    fn new(lines: Vec<String>) -> Self {
        let rows: Vec<Vec<bool>> = lines
            .into_iter()
            .map(|row| {
                row.chars()
                    .map(|c| if c == '@' { true } else { false })
                    .collect()
            })
            .collect();
        let dimensions = (rows.len(), rows[0].len());
        Grid { rows, dimensions }
    }

    fn neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let neighbors = &[
            (point.0.checked_sub(1), point.1.checked_sub(1)),
            (point.0.checked_sub(1), Some(point.1)),
            (point.0.checked_sub(1), Some(point.1 + 1)),
            (Some(point.0), point.1.checked_sub(1)),
            (Some(point.0), Some(point.1 + 1)),
            (Some(point.0 + 1), point.1.checked_sub(1)),
            (Some(point.0 + 1), Some(point.1)),
            (Some(point.0 + 1), Some(point.1 + 1)),
        ];
        neighbors
            .into_iter()
            .filter(|(x, y)| {
                if let (Some(x1), Some(y1)) = (x, y) {
                    *x1 < self.dimensions.0 && *y1 < self.dimensions.1
                } else {
                    false
                }
            })
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
            .collect()
    }

    fn reachable(&self, point: (usize, usize)) -> bool {
        self.neighbors(point)
            .iter()
            .filter(|(x, y)| self.rows[*y][*x])
            .count()
            < 4
    }

    fn remove_reachable(&mut self) -> usize {
        let mut removed = 0;
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                if self.rows[y][x] {
                    if self.reachable((x, y)) {
                        self.rows[y][x] = false;
                        removed += 1;
                    }
                }
            }
        }
        removed
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
    let mut count = 0;
    let grid = Grid::new(vals);
    for (y, row) in grid.rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                if grid.reachable((x, y)) {
                    count += 1;
                }
            }
        }
    }
    println!("Total value is: {count}");
}

fn part_2(vals: Vec<String>) {
    let mut grid = Grid::new(vals);
    let mut total_removed = 0;
    loop {
        let removed = grid.remove_reachable();
        total_removed += removed;
        if removed == 0 {
            break;
        }
    }
    println!("Total value is: {total_removed}");
}
