use std::fs;

#[derive(Copy, Clone, PartialEq)]
enum HandType {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Copy, Clone, PartialEq)]
enum Strategy {
    Lose = 0,
    Draw = 3,
    Win = 6
}

const OPP_ROCK: &str = "A";
const OPP_PAPER: &str = "B";
const OPP_SCISSORS: &str = "C";

const STRAT_ROCK: &str = "X";
const STRAT_PAPER: &str = "Y";
const STRAT_SCISSORS: &str = "Z";

const STRAT_LOSE: &str = "X";
const STRAT_DRAW: &str = "Y";
const STRAT_WIN: &str = "Z";

pub fn solve_day2() {
    println!("[Day2] Reading input.");

    let contents = fs::read_to_string("inputs/day2.txt")
        .expect("[Day2] File read.");

    let lines = contents.split("\n");

    let mut score1 = 0;
    let mut score2 = 0;
    
    for line in lines {
        let letters = line.split(" ").collect::<Vec<&str>>();
        let opponent = letters[0];
        let strategy = letters[1];

        score1 += score_strategy1(opponent, strategy);
        score2 += score_strategy2(opponent, strategy);
    }

    println!("[Day 2.1] Estimated Score is {}", score1);
    println!("[Day 2.2] Estimated Score is {}", score2)
}

fn score_strategy1(opp_letter: &str, strat_letter: &str) -> i32 {
    let opp_hand = parse_opp_hand(opp_letter);
    let strat_hand = parse_strategy_hand(strat_letter);

    return score_game(opp_hand, strat_hand);
}

fn score_strategy2(opp_letter: &str, strat_letter: &str) -> i32 {
    let opp_hand = parse_opp_hand(opp_letter);
    let strategy = parse_strategy_type(strat_letter);

    return if strategy == Strategy::Draw {
        score_game(opp_hand, opp_hand)
    } else if strategy == Strategy::Lose {
        score_game(opp_hand, get_losing_hand(opp_hand))
    } else {
        score_game(opp_hand, get_winning_hand(opp_hand))
    }
}

fn score_game(opp_hand: HandType, strat_hand: HandType) -> i32 {
    let game_score = score_matchup(opp_hand, strat_hand);
    return game_score + (strat_hand as i32)
}

fn score_matchup(opp_hand: HandType, strat_hand: HandType) -> i32 {
    if opp_hand == strat_hand {
        return 3
    }
    if get_losing_hand(opp_hand) == strat_hand {
        return 0
    }
    if get_winning_hand(opp_hand) == strat_hand {
        return 6
    }

    return 0
}

fn parse_opp_hand(letter: &str) -> HandType {
    return if letter.eq(OPP_ROCK) {
        HandType::Rock
    } else if letter.eq(OPP_PAPER) {
        HandType::Paper
    } else {
        HandType::Scissors
    }
}

fn parse_strategy_hand(letter: &str) -> HandType {
    return if letter.eq(STRAT_ROCK) {
        HandType::Rock
    } else if letter.eq(STRAT_PAPER) {
        HandType::Paper
    } else {
        HandType::Scissors
    }
}

fn parse_strategy_type(letter: &str) -> Strategy {
    return if letter.eq(STRAT_LOSE) {
        Strategy::Lose
    } else if letter.eq(STRAT_DRAW) {
        Strategy::Draw
    } else {
        Strategy::Win
    }
}

fn get_winning_hand(hand: HandType) -> HandType {
    return if hand == HandType::Rock {
        HandType::Paper
    } else if hand == HandType::Paper {
        HandType::Scissors
    } else {
        HandType::Rock
    }
}

fn get_losing_hand(hand: HandType) -> HandType {
    return if hand == HandType::Rock {
        HandType::Scissors
    }
    else if hand == HandType::Paper {
        HandType::Rock
    }
    else {
        return HandType::Paper
    }
}