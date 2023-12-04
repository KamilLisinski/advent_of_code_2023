use std::{io::{self}, collections::HashMap};

fn parse_line(s: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    let prefix = "Card ";

    if let Some((g, values)) = s.split_once(":") {
        if let Some(id) = g.strip_prefix(prefix).and_then(|s| Some(s.trim())).and_then(|s| s.parse::<usize>().ok()) {

            if let Some((winning_s, numbers_s)) = values.split_once('|') {
                let winning_n: Option<Vec<usize>> = winning_s.trim().split_ascii_whitespace().into_iter().map(|s| s.parse::<usize>().ok()).collect();
                let numbers_n: Option<Vec<usize>> = numbers_s.trim().split_ascii_whitespace().into_iter().map(|s| s.parse::<usize>().ok()).collect();

                if let (Some(w), Some(n)) = (winning_n, numbers_n) {
                    return Some((id, w, n));
                }
            }
        }
    }

    return None;
}

fn get_score(winning: &[usize], numbers: &[usize]) -> Option<usize> {
    // Scanning arrays is fast (in real world ususally faster than building sets, etc. for small arrays)
    let c = numbers.iter().filter(|n| winning.contains(n)).count();
    if c > 0 {
        return Some(c);
    }
    return None;
}


fn main() {
    let mut total_score = 0;

    let mut counts: HashMap<usize, usize> = HashMap::new();

    for line in io::stdin().lines() {
        let uline = line.unwrap();
        if let Some((id, w, n)) = parse_line(&uline) {
            let current_count = *counts.entry(id).or_insert(1);
            if let Some(score) = get_score(&w, &n) {
                total_score += 2_usize.pow((score - 1).try_into().unwrap());
                for i in 0..score {
                    let next_card_count = counts.entry(id + 1 + i).or_insert(1);
                    *next_card_count += current_count;
                }
            }
        }
        else {
            println!("line parse failed: {}", uline);
        }
    }
    println!("part 1: {}", total_score);
    println!("part 2: {}", counts.values().sum::<usize>());
}


#[test]
fn test_parse_line() {
    let res: Option<(usize, Vec<usize>, Vec<usize>)> = parse_line("Card 2: 1  2 3 | 4 5 6");
    assert_eq!(res.is_some(), true);
    let (s, w, n) = res.unwrap();
    assert_eq!(s, 2);
    
    assert_eq!(w[0], 1);
    assert_eq!(w[1], 2);
    assert_eq!(w[2], 3);

    assert_eq!(n[0], 4);
    assert_eq!(n[1], 5);
    assert_eq!(n[2], 6);
}

#[test]
fn test_get_score_exponent() {
    assert_eq!(get_score(&[1, 2, 3], &[1, 2, 3]).unwrap(), 2);
    assert_eq!(get_score(&[1, 2, 3], &[1, 2]).unwrap(), 1);
    assert_eq!(get_score(&[1, 2, 3], &[1]).unwrap(), 0);
    assert_eq!(get_score(&[1, 2, 3], &[4, 5, 6]).is_none(), true);
}