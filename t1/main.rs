use std::io::{self};


static DIGITS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];


fn forward_read(s: &str) -> u32 {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap().is_digit(10) {
            return s.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
        else {
            // Not optimal
            // 1. If this is the strategy we should match at most 5 characters back
            // 2. Should be done by a tree to have better
            for j in 1..=DIGITS.len() {
                if s[0..i+1].ends_with(DIGITS[j-1]) {
                    return j.try_into().unwrap();
                }
            }
        }
    }
    return 0;
}

fn backward_read(s: &str) -> u32 {
    for i in (0..s.len()).rev() {
        if s.chars().nth(i).unwrap().is_digit(10) {
            return s.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
        else {
            for j in 1..=DIGITS.len() {
                if s[i..s.len()].starts_with(DIGITS[j-1]) {
                    return j.try_into().unwrap();
                }
            }
        }
    }
    return 0;
}

fn main() -> io::Result<()> {
    let mut sum: u32 = 0;

    for line in io::stdin().lines() {
        let line_value = line.unwrap();
        sum += forward_read(&line_value) * 10;
        sum += backward_read(&line_value);
    }

    println!("{}", sum);
    Ok(())
}


// Unit tests

#[test]
fn test_forward_read() {
    assert_eq!(forward_read("abc123def456"), 1);
    assert_eq!(forward_read("abc23def456"), 2);
    assert_eq!(forward_read("abc3def456"), 3);
    assert_eq!(forward_read("abcdef456"), 4);
    assert_eq!(forward_read("abcdef56"), 5);
    assert_eq!(forward_read("onedef56"), 1);
    assert_eq!(forward_read("abctwodef56"), 2);
    assert_eq!(forward_read("abcdefthree56"), 3);
}

#[test]
fn test_backward_read() {
    assert_eq!(backward_read("abc123def456"), 6);
    assert_eq!(backward_read("onetwothreeabc123def456"), 6);
    assert_eq!(backward_read("abc123def45"), 5);
    assert_eq!(backward_read("abc123def4"), 4);
    assert_eq!(backward_read("abc123def"), 3);
    assert_eq!(backward_read("abc123defive"), 5);
    assert_eq!(backward_read("abc123deoneight"), 8);
    assert_eq!(backward_read("abc123deoneigh"), 1);
}