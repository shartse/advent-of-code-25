use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    part_1(values.clone());
    part_2(values);
}

#[derive(Debug)]
struct WorkSheet {
    numbers: Vec<Vec<u64>>,
    ops: Vec<char>,
}

impl WorkSheet {
    fn new(lines: Vec<String>) -> Self {
        let mut numbers = Vec::new();
        let mut ops = Vec::new();
        for l in lines {
            let mut row = Vec::new();
            for each in l.split_ascii_whitespace() {
                if each == "+" {
                    ops.push('+');
                } else if each == "*" {
                    ops.push('*');
                } else {
                    row.push(each.parse::<u64>().unwrap());
                }
            }
            if row.len() != 0 {
                numbers.push(row);
            }
        }
        WorkSheet { numbers, ops }
    }

    fn col(&self, n: usize) -> Vec<u64> {
        self.numbers.iter().map(|row| row[n]).collect()
    }

    fn solve(&self, n: usize) -> u64 {
        if self.ops[n] == '+' {
            self.col(n).iter().sum()
        } else {
            self.col(n).iter().product()
        }
    }
}

fn part_1(vals: Vec<String>) {
    let mut total_ans = 0;
    let worksheet = WorkSheet::new(vals);
    for i in 0..worksheet.numbers[0].len() {
        let ans = worksheet.solve(i);
        total_ans += ans;
    }
    println!("Total value is: {total_ans}");
}

#[derive(Debug)]
struct SpecialWorkSheet {
    numbers: Vec<Vec<char>>,
    ops: Vec<char>,
}

impl SpecialWorkSheet {
    fn new(lines: Vec<String>) -> Self {
        let mut numbers = Vec::new();
        let mut ops = Vec::new();
        let mut ops_row = false;
        for l in lines.iter() {
            let mut row = Vec::new();
            for c in l.chars() {
                if c == '+' || c == '*' {
                    ops.push(c);
                    ops_row = true;
                } else {
                    row.push(c);
                }
            }
            if !ops_row {
                numbers.push(row);
            }
        }
        SpecialWorkSheet { numbers, ops }
    }
}

fn part_2(vals: Vec<String>) {
    let mut ans = 0;
    let worksheet = SpecialWorkSheet::new(vals);
    let cols = worksheet.ops.len();
    let row_len = worksheet.numbers[0].len();
    let mut i = 0;
    for c in 0..cols {
        let mut nums = Vec::new();
        loop {
            if i >= row_len {
                break;
            }
            let x = worksheet
                .numbers
                .iter()
                .map(|row| row[i])
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u64>();
            i += 1;
            if let Ok(n) = x {
                nums.push(n);
            } else {
                break;
            }
        }
        let val: u64 = if worksheet.ops[c] == '+' {
            nums.iter().sum()
        } else {
            nums.iter().product()
        };
        ans += val;
    }
    println!("Total value is: {ans}");
}
