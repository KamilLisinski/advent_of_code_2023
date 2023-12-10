use std::io;
use std::iter::zip;


fn parse_line(s: &str) -> Option<Vec::<usize>> {
    if let Some((_, numbers)) = s.split_once(":") {
        return Some(numbers.trim().split_whitespace().filter_map(|n| n.parse::<usize>().ok()).collect());
    }
    return None;
}


fn find_x_above(a: usize, b: usize) -> i64 {
    return (((((a*a-4*b) as f64).sqrt() + (a as f64))/2f64).ceil() - (((a as f64-((a*a-4*b) as f64).sqrt())/2f64)+0.001f64).ceil()).floor() as i64;
}

fn main() {
    // let mut total_score = 0;

    // let mut counts: HashMap<usize, usize> = HashMap::new();

    let race_lines: Vec<Vec<usize>> = io::stdin().lines().filter_map(|l| l.ok()).filter_map(|l| parse_line(&l)).collect();

    if race_lines.len() == 2 {
        let mut mult: i64 = 1;

        for (a, b) in zip(&race_lines[0], &race_lines[1]) {
            mult = mult * find_x_above(*a, *b);
        }
        println!("{}", mult);
    }
}



#[test]
fn test_parse_line() {
    let res = parse_line("Time:      7  15   30");
    assert_eq!(res.is_some(), true);
    let v = res.unwrap();
    
    assert_eq!(v[0], 7);
    assert_eq!(v[1], 15);
    assert_eq!(v[2], 30);
    assert_eq!(v.len(), 3);
}

#[test]
fn test_find_x_above() {
    println!("{}", find_x_above(7, 9));
    println!("{}", find_x_above(15, 40));
    println!("{}", find_x_above(30, 200));
    assert!(find_x_above(7, 9) == 4);
    assert!(find_x_above(15, 40) == 8);
    assert!(find_x_above(30, 200)== 9);
}