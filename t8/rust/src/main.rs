use std::{
    collections::{HashMap, HashSet},
    io,
};

use num::Integer;

fn parse_str_to_int(s: &str) -> Option<u64> {
    if s.len() != 3 {
        return None;
    }
    let base = ('Z' as u32 - 'A' as u32) as u32 + 1;
    let mut sum: u64 = 0;
    for (i, c) in s.chars().enumerate() {
        let exponent: u32 = i.try_into().unwrap();
        sum = sum + ((c as u32 - 'A' as u32) * base.pow(2 - exponent)) as u64;
    }
    return Some(sum);
}

fn parse_line(s: &str) -> Option<(u64, u64, u64)> {
    if let Some((first, tail)) = s.split_once(" = ") {
        if let Some(first_parsed) = parse_str_to_int(first) {
            if let Some((Some(second_parsed), Some(third_parsed))) = tail
                .strip_prefix("(")
                .and_then(|t| t.strip_suffix(")"))
                .and_then(|t| t.split_once(", "))
                .map(|p| (parse_str_to_int(p.0), parse_str_to_int(p.1)))
            {
                return Some((first_parsed, second_parsed, third_parsed));
            }
        }
    }
    return None;
}

struct Choice {
    left: u64,
    right: u64,
}

fn build_map(v: &[(u64, u64, u64)]) -> HashMap<u64, Choice> {
    let mut map: HashMap<u64, Choice> = HashMap::new();

    for (key, left, right) in v {
        map.insert(
            *key,
            Choice {
                left: *left,
                right: *right,
            },
        );
    }

    return map;
}

fn traverse(directions: &str, map: &HashMap<u64, Choice>) -> usize {
    let mut count = 0;
    let end_node = parse_str_to_int("ZZZ").unwrap();
    let mut current_node = parse_str_to_int("AAA").unwrap();
    loop {
        for direction in directions.chars() {
            count = count + 1;
            if direction == 'L' {
                current_node = map.get(&current_node).unwrap().left;
            } else {
                current_node = map.get(&current_node).unwrap().right;
            }
            if current_node == end_node {
                return count;
            }
        }
    }
}

fn multi_traverse(directions: &str, map: &HashMap<u64, Choice>) -> usize {
    let base = ('Z' as u32 - 'A' as u32) as u32 + 1;
    let mut start_nodes: Vec<u64> = map
        .keys()
        .filter(|v| *v % (base as u64) == 0)
        .map(|v| *v)
        .collect();
    let mut counts: Vec<usize> = Vec::new();
    for node in start_nodes.iter_mut() {
        let mut count = 0;
        'outer: loop {
            for direction in directions.chars() {
                count = count + 1;
                if direction == 'L' {
                    *node = map.get(&node).unwrap().left;
                } else {
                    *node = map.get(&node).unwrap().right;
                }
                if (*node % (base as u64)) == ((base as u64) - 1) {
                    counts.push(count);
                    break 'outer;
                }
            }
        }
    }
    let mut lcm:usize = 1;
    for n in counts {
        lcm = lcm.lcm(&n);
    }
    return lcm;
}

fn main() {
    let mut directions: Option<String> = None;
    let mut values: Vec<(u64, u64, u64)> = Vec::new();
    for line in io::stdin().lines() {
        if directions.is_none() {
            directions = Some(line.unwrap());
            continue;
        }
        let uline = line.unwrap();
        if uline.len() == 0 {
            continue;
        }

        if let Some(parsed) = parse_line(&uline) {
            values.push(parsed);
        }
    }
    let map = build_map(&values);

    let u_diretions = directions.unwrap();
    println!("part1: {}", traverse(&u_diretions, &map));
    println!("part2: {}", multi_traverse(&u_diretions, &map));
}

#[test]
fn test_parse_str_to_int() {
    // We should get all unique numbers for all 3 'digit' strings
    let mut count = 0;
    let mut set: HashSet<u64> = HashSet::new();
    let base = ('Z' as u32 - 'A' as u32) as u32 + 1;
    for i1 in 0..base {
        let c1: char = ('A' as u32 + i1).try_into().unwrap();
        for i2 in 0..base {
            let c2: char = ('A' as u32 + i2).try_into().unwrap();
            for i3 in 0..base {
                let c3: char = ('A' as u32 + i3).try_into().unwrap();
                let s = String::from_iter([c1, c2, c3]);
                count = count + 1;
                set.insert(parse_str_to_int(&s).unwrap());
            }
        }
    }
    assert_eq!(count, set.len());
    assert_eq!(parse_str_to_int("AAA").unwrap() as u32 % base, 0);
    assert_eq!(parse_str_to_int("ABA").unwrap() as u32 % base, 0);
    assert_eq!(parse_str_to_int("CDA").unwrap() as u32 % base, 0);
    assert_eq!(parse_str_to_int("EFA").unwrap() as u32 % base, 0);
    assert_eq!(parse_str_to_int("AAZ").unwrap() as u32 % base, base - 1);
    assert_eq!(parse_str_to_int("BCZ").unwrap() as u32 % base, base - 1);
    assert_eq!(parse_str_to_int("DFZ").unwrap() as u32 % base, base - 1);
    assert_eq!(parse_str_to_int("EZZ").unwrap() as u32 % base, base - 1);
    assert_eq!(parse_str_to_int("ZZZ").unwrap() as u32 % base, base - 1);
}

#[test]
fn test_parse_line() {
    assert!(parse_line("AAA = (BBB, CCC)").is_some());
}
