use std::io::{self};

struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn add_value(&mut self, v: u32, s: &str) {
        match s {
            "blue" => self.blue += v,
            "red" => self.red += v,
            "green" => self.green += v,
            &_ => ()
        }
    }
}

fn parse_line(s: &str) -> Option<(u32, Vec<Game>)> {
    let parts: Vec<&str> = s.split(":").collect();

    // let mut id: Option<u32> = None;

    let prefix = "Game ";

    // if parts.get(0).is_some_and(|s| s.starts_with(prefix)) {
    //     let id_str: Option<&str> = parts.get(0).unwrap().get(prefix.len()..);
    //     if let Some(id_str_u) = id_str {
    //         id = id_str_u.parse::<u32>().ok();
    //     }
    // }

    if let Some(id) = parts.get(0).and_then(|s: &&str| -> Option<&str> {
        if s.starts_with(prefix) {
            return s.get(prefix.len()..);
        }
        else {
            None
        }
    }).and_then(|s: &str| s.parse::<u32>().ok()) {
        if let Some(games) = parts.get(1) {
            Some((id, games.split(";").map(|g: &str| parse_game(g)).collect()))
        }
        else {
            None
        }
    }
    else {
        None
    }


}

fn parse_game(s: &str) -> Game {
    let mut game = Game {
        red: 0,
        blue: 0,
        green: 0
    };

    let trimmed: &str = s.trim();

    for part in trimmed.split(",") {
        let trimmed_part: &str = part.trim();

        let parts: Vec<&str> = trimmed_part.split(" ").collect();

        if parts.len() != 2 {
            continue;
        }
        else {
            let count = parts.get(0).unwrap().parse::<u32>();
            if count.is_err() {
                continue;
            }
            game.add_value(count.unwrap(), parts.get(1).unwrap());
        }
    }
    return game;
}


fn check_games(games: &[Game], ref_game: &Game) -> bool {
    for game in games.iter() {
        if game.blue > ref_game.blue || game.red > ref_game.red || game.green > ref_game.green {
            return false;
        }
    }
    return true;
}


fn main() {
    let mut id_sum = 0;
    let ref_game = Game {red: 12, blue: 14, green: 13};

    for line in io::stdin().lines() {
        let uline = line.unwrap();
        if let Some(res) = parse_line(&uline) {
            if check_games(&res.1, &ref_game) {
                id_sum += res.0;
            }
        }
    }
    println!("{}", id_sum);
}

#[test]
fn test_parse_game() {
    let game: Game = parse_game(" 3 blue, 4 red");
    assert_eq!(game.red, 4);
    assert_eq!(game.green, 0);
    assert_eq!(game.blue, 3);
}

#[test]
fn test_parse_line() {
    let res: Option<(u32, Vec<Game>)> = parse_line("Game 2: 3 blue, 4 red; 1 red, 1 blue, 1 green");
    assert_eq!(res.is_some(), true);
    let unwrapped = res.unwrap();
    assert_eq!(unwrapped.0, 2);
    match unwrapped.1.get(0) {
        Some(g) => {
            assert_eq!(g.blue, 3);
            assert_eq!(g.red, 4);
            assert_eq!(g.green, 0);
        },
        None => panic!("should exist")
    }
    match unwrapped.1.get(1) {
        Some(g) => {
            assert_eq!(g.blue, 1);
            assert_eq!(g.red, 1);
            assert_eq!(g.green, 1);
        },
        None => panic!("should exist")
    }  
}

#[test]
fn test_check_games() {
    let good_games: Vec<Game> = vec![Game{red: 1, blue: 1, green: 1}];
    let bad_games: Vec<Game> = vec![Game{red: 1, blue: 3, green: 1}];
    let ref_game: Game = Game{red: 2, blue: 2, green: 2};

    assert_eq!(check_games(&good_games, &ref_game), true);
    assert_eq!(check_games(&bad_games, &ref_game), false);
}