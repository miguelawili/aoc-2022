use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_lines(filepath: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn get_score_map() -> HashMap<Move, u32> {
    let mut move_scores: HashMap<Move, u32> = HashMap::new();
    move_scores.insert(Move::Rock, 1);
    move_scores.insert(Move::Paper, 2);
    move_scores.insert(Move::Scissors, 3);

    move_scores
}

fn get_own_move_map() -> HashMap<char, Move> {
    let mut move_map: HashMap<char, Move> = HashMap::new();
    move_map.insert('X', Move::Rock);
    move_map.insert('Y', Move::Paper);
    move_map.insert('Z', Move::Scissors);

    move_map
}

fn get_outcome_map() -> HashMap<char, Outcome> {
    let mut outcome_map: HashMap<char, Outcome> = HashMap::new();
    outcome_map.insert('X', Outcome::Lose);
    outcome_map.insert('Y', Outcome::Draw);
    outcome_map.insert('Z', Outcome::Win);

    outcome_map
}

fn get_outcome_fn_map() -> HashMap<Outcome, fn(&Move) -> Move> {
    let mut outcome_map: HashMap<Outcome, fn(&Move) -> Move> = HashMap::new();
    outcome_map.insert(Outcome::Lose, get_losing_move);
    outcome_map.insert(Outcome::Draw, get_drawing_move);
    outcome_map.insert(Outcome::Win, get_winning_move);

    outcome_map
}

fn get_enemy_move_map() -> HashMap<char, Move> {
    let mut move_map: HashMap<char, Move> = HashMap::new();
    move_map.insert('A', Move::Rock);
    move_map.insert('B', Move::Paper);
    move_map.insert('C', Move::Scissors);

    move_map
}

struct BattlingMoves<'a> {
    own_move: &'a Move,
    enemy_move: &'a Move,
}

fn score_move(battle_moves: &BattlingMoves) -> u32 {
    let own_move = battle_moves.own_move;
    let enemy_move = battle_moves.enemy_move;
    let rock = &Move::Rock;
    let paper = &Move::Paper;
    let scissors = &Move::Scissors;

    if own_move == enemy_move {
        return 3;
    } else if own_move == rock && enemy_move == scissors {
        return 6;
    } else if own_move == scissors && enemy_move == paper {
        return 6;
    } else if own_move == paper && enemy_move == rock {
        return 6;
    } else {
        return 0;
    }
}

fn get_winning_move(enemy_move: &Move) -> Move {
    let rock = &Move::Rock;
    let paper = &Move::Paper;

    if enemy_move == rock {
        Move::Paper
    } else if enemy_move == paper {
        Move::Scissors
    } else {
        Move::Rock
    }
}

fn get_losing_move(enemy_move: &Move) -> Move {
    let rock = &Move::Rock;
    let paper = &Move::Paper;

    if enemy_move == rock {
        Move::Scissors
    } else if enemy_move == paper {
        Move::Rock
    } else {
        Move::Paper
    }
}

fn get_drawing_move(enemy_move: &Move) -> Move {
    let rock = &Move::Rock;
    let paper = &Move::Paper;

    if enemy_move == rock {
        Move::Rock
    } else if enemy_move == paper {
        Move::Paper
    } else {
        Move::Scissors
    }
}

fn main() {
    let mut p1_total_score = 0;
    let score_map = get_score_map();
    let own_move_map = get_own_move_map();
    let enemy_move_map = get_enemy_move_map();

    let mut p2_total_score = 0;
    let outcome_map = get_outcome_map();
    let outcome_fn_map = get_outcome_fn_map();

    if let Ok(lines) = read_lines("./day2input.txt") {
        for line in lines {
            match line {
                Ok(line) => {
                    println!("------------------------");
                    let mut iter = line.split_whitespace();

                    let enemy_move_str = iter.nth(0).expect("Can't find 0th index str.");
                    let own_move_str = iter.nth(0).expect("Can't find 0th index str.");

                    let enemy_move_char =
                        &enemy_move_str.chars().next().expect("Can't parse char.");
                    let own_move_char = &own_move_str.chars().next().expect("Can't parse char.");

                    println!("str: {} <-> {}", enemy_move_str, own_move_str);
                    println!("char: {} <-> {}", enemy_move_char, own_move_char);

                    let own_move: &Move = own_move_map
                        .get(own_move_char)
                        .expect("Can't get equivalent move.");
                    let enemy_move: &Move = enemy_move_map
                        .get(enemy_move_char)
                        .expect("Can't get equivalent move.");

                    let battle_moves = BattlingMoves {
                        own_move,
                        enemy_move,
                    };

                    let battle_result = score_move(&battle_moves);
                    let move_score = score_map.get(own_move).expect("Can't map move to score!");

                    println!("move_score: {}", move_score);
                    println!("battle_result: {}", battle_result);
                    let to_add = battle_result + move_score;

                    println!("total_score: {}", p1_total_score);
                    p1_total_score += to_add;

                    let intended_outcome = outcome_map
                        .get(own_move_char)
                        .expect("Can't map char to outcome enum.");
                    let fn_to_call = outcome_fn_map
                        .get(intended_outcome)
                        .expect("Can't map outcome to fn call.");

                    let intended_move = fn_to_call(enemy_move);

                    let battle_moves = BattlingMoves {
                        own_move: &intended_move,
                        enemy_move,
                    };

                    let battle_result = score_move(&battle_moves);
                    let move_score = score_map
                        .get(&intended_move)
                        .expect("Can't map move to score!");

                    println!("move_score: {}", move_score);
                    println!("battle_result: {}", battle_result);
                    let to_add = battle_result + move_score;

                    println!("total_score: {}", p1_total_score);
                    p2_total_score += to_add;
                }
                _ => {}
            }
        }
    }

    println!("========================");
    println!("p1 total score: {}", p1_total_score);
    println!("p2 total score: {}", p2_total_score);
}
