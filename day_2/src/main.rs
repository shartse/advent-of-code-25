use std::{
    cmp::max,
    collections::HashSet,
    fs::File,
    io::{self, Read},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut s = String::new();
    io::BufReader::new(file).read_to_string(&mut s).unwrap();
    let values: Vec<&str> = s.split(',').collect();
    part_1(values.clone());
    part_2(values.clone());
}

fn invalid_ids(a: &str, b: &str, n: usize) -> Vec<i64> {
    let mut ids = Vec::new();
    let mut starting_len = a.len() as u32;
    if starting_len % 2 == 1 {
        starting_len += 1
    }
    let exp = max(1, starting_len / n as u32) - 1;
    let testing_value = i64::pow(10, exp);
    let starting_value = a.trim().parse::<i64>().unwrap();
    let ending_value = b.trim().parse::<i64>().unwrap();
    for i in testing_value..ending_value {
        let s = i.to_string().repeat(n);
        let possible_val = s.parse::<i64>().unwrap();
        if possible_val > ending_value {
            break;
        }
        if possible_val >= starting_value {
            ids.push(possible_val);
        }
    }
    return ids;
}

fn part_1(vals: Vec<&str>) {
    let mut sum = 0;
    for v in vals.into_iter() {
        let (a, b) = v.split_once('-').unwrap();
        let ids = invalid_ids(a, b, 2);
        for i in ids {
            sum += i
        }
    }
    println!("Total value is: {sum}");
}

fn part_2(vals: Vec<&str>) {
    let mut all_ids = HashSet::new();
    for v in vals.into_iter() {
        let (a, b) = v.split_once('-').unwrap();
        for n in 2..(b.len() + 1) {
            let ids = invalid_ids(a, b, n);
            for i in ids {
                all_ids.insert(i);
            }
        }
    }
    let sum: i64 = all_ids.iter().sum();
    println!("Total value is: {sum}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn part_2() {
        panic!("Make this test fail");
    }
}
