use std::{io, u32, cmp};

struct PartNumber {
    value: u32,
    start_index: usize,
    length: usize,
}


fn get_all_numbers(line: &str) -> Vec<PartNumber> {
    let mut numbers: Vec<PartNumber> = Vec::new();

    let mut current_index: Option<usize> = None;
    let mut length: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            if current_index.is_none() {
                current_index = Some(i);
            }
            length += 1;
        }
        else {
            if let Some(c) = current_index {
                if let Some(v) = line.get(c..i).and_then(|s: &str| s.parse::<u32>().ok()) {
                    numbers.push(PartNumber { value: v, start_index: c, length: length });
                }

                length = 0;
                current_index = None;
            }
        }
    }
    if let Some(c) = current_index {
        if let Some(v) = line.get(c..).and_then(|s: &str| s.parse::<u32>().ok()) {
            numbers.push(PartNumber { value: v, start_index: c, length: length });
        }
    }

    return  numbers;
}

impl PartNumber {
    fn check_if_part<T: AsRef<str>>(&self, lines: &[T]) -> bool {
        for line in lines {
            if let Some(s) = line.as_ref().get((self.start_index.checked_sub(1).unwrap_or(0))..cmp::min(self.start_index + self.length + 1, line.as_ref().len())) {
                for c in s.chars() {
                    if !c.is_ascii_digit() && c != '.' {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

fn main() {
    let mut sum_of_parts: u32 = 0;

    let lines: Vec<String> = io::stdin().lines().map(|l| l.unwrap()).collect();

    for (i, line) in lines.iter().enumerate() {
        let parts: Vec<PartNumber> = get_all_numbers(&line);
        let context_lines = lines.get(i.checked_sub(1).unwrap_or(0)..(cmp::min(lines.len(), i+2)));

        if let Some(cl) = context_lines {
            for part in parts.iter() {
                if part.check_if_part(cl) {
                    sum_of_parts += part.value;
                }
            }
        }
    }

    println!("part 1: {}", sum_of_parts);
}

#[test]
fn test_get_all_numbers() {
    let nums: Vec<PartNumber> = get_all_numbers("....123...456.7");
    assert_eq!(nums[0].value, 123);
    assert_eq!(nums[0].start_index, 4);
    assert_eq!(nums[0].length, 3);
    assert_eq!(nums[1].value, 456);
    assert_eq!(nums[1].start_index, 10);
    assert_eq!(nums[1].length, 3);
    assert_eq!(nums[2].value, 7);
    assert_eq!(nums[2].start_index, 14);
    assert_eq!(nums[2].length, 1);
}

#[test]
fn test_check_if_part() {
    let part: PartNumber = PartNumber { value: 1, start_index: 0, length: 1 };
    let bad_lines: Vec<&str> = vec!["...", "1..", "..."];
    assert_eq!(part.check_if_part(&bad_lines), false);
    let good_lines: Vec<&str> = vec![".@.", "1..", "..."];
    assert_eq!(part.check_if_part(&good_lines), true);
}

#[test]
fn test_check_if_part_2() {
    let part: PartNumber = PartNumber { value: 467, start_index: 0, length: 3 };
    let good_lines: Vec<&str> = vec!["467..114..", "...*......"];
    assert_eq!(part.check_if_part(&good_lines), true);
}