use iter_first_max::IterFirstMaxExt as _;
use std::{
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let values: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    solve(values.clone(), 2);
    solve(values, 12);
}

fn largest_right_of(i: usize, v: &Vec<u32>) -> Option<(usize, &u32)> {
    let (pos, v) = v[i..]
        .iter()
        .enumerate()
        .first_max_by_key(|&(_, value)| value)
        .unwrap();
    if *v == 0 {
        None
    } else {
        Some((pos + i, v))
    }
}

fn solve(vals: Vec<String>, n: usize) {
    let mut ans = 0;
    for v in vals {
        let mut vals: Vec<(usize, u32)> = Vec::new();
        let mut batteries: Vec<u32> = v.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut starting_pos = 0;
        for i in 0..n {
            loop {
                if let Some((pos, max)) = largest_right_of(starting_pos, &batteries) {
                    vals.push((pos, *max));
                    batteries[pos] = 0;
                    starting_pos = pos;
                    break;
                } else {
                    // go to the right-most zero that has something other than zeros following it
                    let mut new_start = batteries.len() - 1; 
                    loop{
                        if new_start == 0 || (batteries[new_start] == 0 && batteries[new_start..].iter().sum::<u32>() != 0) {
                            break;
                        }
                        new_start = new_start - 1;

                    }
                    starting_pos = new_start;
                }
            }
        }
        vals.sort_by(|(idx_a, _), (idx_b, _)| idx_a.partial_cmp(idx_b).unwrap());
        let final_number: u64 = vals
            .iter()
            .map(|(_, val)| val.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();
        ans += final_number;
    }
    println!("Total value is: {ans}");
}