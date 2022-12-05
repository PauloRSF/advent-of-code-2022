use std::{fs::read_to_string, panic};

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_player_representation(shape_char: &char) -> Self {
        match shape_char {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!(
                "Character {} isn't a valid shape representation for the player",
                shape_char
            ),
        }
    }

    fn from_opponent_representation(shape_char: &char) -> Self {
        match shape_char {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!(
                "Character {} isn't a valid shape representation for the opponent",
                shape_char
            ),
        }
    }

    fn score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl RoundResult {
    fn from(result_char: &char) -> Self {
        match result_char {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(
                "Character {} isn't a valid round result representation",
                result_char
            ),
        }
    }
}

fn get_round_score(opponent_shape: &Shape, player_shape: &Shape) -> u8 {
    let match_score = match (opponent_shape, player_shape) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Rock, Shape::Scissors) => 0,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Scissors, Shape::Scissors) => 3,
    };

    match_score + player_shape.score()
}

fn get_shape_to_play(opponent_shape: &Shape, desired_result: &RoundResult) -> Shape {
    match (opponent_shape, desired_result) {
        (Shape::Rock, RoundResult::Win) => Shape::Paper,
        (Shape::Rock, RoundResult::Draw) => Shape::Rock,
        (Shape::Rock, RoundResult::Lose) => Shape::Scissors,
        (Shape::Paper, RoundResult::Win) => Shape::Scissors,
        (Shape::Paper, RoundResult::Draw) => Shape::Paper,
        (Shape::Paper, RoundResult::Lose) => Shape::Rock,
        (Shape::Scissors, RoundResult::Win) => Shape::Rock,
        (Shape::Scissors, RoundResult::Draw) => Shape::Scissors,
        (Shape::Scissors, RoundResult::Lose) => Shape::Paper,
    }
}

fn calculate_score_with_misinterpreted_guide(round_chars: &[(char, char)]) -> u32 {
    round_chars
        .iter()
        .map(|(opponent_shape_char, player_shape_char)| {
            get_round_score(
                &Shape::from_opponent_representation(opponent_shape_char),
                &Shape::from_player_representation(player_shape_char),
            ) as u32
        })
        .sum()
}

fn calculate_score_with_correct_guide(round_chars: &[(char, char)]) -> u32 {
    round_chars
        .iter()
        .map(|(opponent_shape_char, desired_result)| {
            let player_shape_to_play = get_shape_to_play(
                &Shape::from_opponent_representation(opponent_shape_char),
                &RoundResult::from(desired_result),
            );

            get_round_score(
                &Shape::from_opponent_representation(opponent_shape_char),
                &player_shape_to_play,
            ) as u32
        })
        .sum()
}

fn main() {
    let input = read_to_string("./input.txt").expect("Input file should be readable");

    let round_chars = input
        .trim()
        .split('\n')
        .map(|round_line| round_line.chars().collect::<Vec<_>>())
        .map(|round_chars| match round_chars[..] {
            [opponent_shape_char, _, player_shape_char] => (opponent_shape_char, player_shape_char),
            _ => panic!("Malformed round chars: {:?}", round_chars),
        })
        .collect::<Vec<_>>();

    let misinterpreted_guide_final_score = calculate_score_with_misinterpreted_guide(&round_chars);

    println!(
        "Assuming that the second column is the opponent's move, the guide should warrant a final score of {} points",
        misinterpreted_guide_final_score
    );

    let actual_guide_final_score = calculate_score_with_correct_guide(&round_chars);

    println!(
        "Correctly decrypting it, the guide should warrant a final score of {} points",
        actual_guide_final_score
    );
}
