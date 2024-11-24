use std::str;

use inquire::{Confirm, Select};

use crate::app::field::{Row, Spot};

use super::{
    card::Card,
    field::{Field, RowOfCards},
    player::{Hand, Player},
    AffectedCards, Op, Play, Score, Season, WinCondition,
};

pub(crate) fn get_num_players() -> i32 {
    loop {
        let res = Select::new("Select number of players: ", vec![2, 3, 4]).prompt();
        if let Ok(num_players) = res {
            return num_players;
        }
    }
}
pub(crate) fn select_play(player_turn: usize, players: &Vec<Player>) -> Play {
    let seasons: Vec<Season> = players.iter().map(|p| p.season()).collect();
    for i in 0..players.len() {
        if i == player_turn {
            continue;
        }
        show_title(players[i].season().to_string().as_str());
        show_field(players[i].field());
    }
    show_title("Your Field");
    show_field(players[player_turn].field());

    let hand = players[player_turn].hand();

    loop {
        let card_index = select_card_from_hand(hand);
        let card_name = hand[card_index].to_text();
        let is_swap = hand[card_index].rune().ability().is_swap();
        if is_swap {
            'field: loop {
                let player_field = select_player_field(seasons.clone(), &card_name);
                match player_field {
                    Some(idx) => 'spot1: loop {
                        let opt_spot =
                            select_spot_on_field(players[idx].field(), is_swap, &card_name);
                        match opt_spot {
                            Some(spot) => {
                                return Play {
                                    played_field: idx,
                                    player_turn,
                                    card_index,
                                    spot,
                                }
                            }
                            None => break 'spot1,
                        }
                    },
                    None => break 'field,
                }
            }
        } else {
            'spot2: loop {
                let opt_spot =
                    select_spot_on_field(players[player_turn].field(), is_swap, &card_name);
                match opt_spot {
                    Some(spot) => {
                        return Play {
                            played_field: player_turn,
                            player_turn,
                            card_index,
                            spot,
                        }
                    }
                    None => break 'spot2,
                }
            }
        }
    }
}
pub(crate) fn choose_prize(winner: usize, prizes: Vec<&Card>, seasons: Vec<Season>) -> usize {
    let options: Vec<String> = (0..seasons.len())
        .map(|i| {
            let c = prizes[i];
            let card_description = format!(
                "{} {} {}/{}",
                c.season(),
                c.rune(),
                c.garden_score(),
                c.court_score()
            );
            if i == winner {
                format!("Your own {}", card_description)
            } else {
                format!("{} player's {}", seasons[i], card_description)
            }
        })
        .collect();

    Select::new("Which prize will you take?", options)
        .raw_prompt()
        .expect("Should make a choice.")
        .index;
    todo!()
}
pub(crate) fn game_over(winner_season: Season, condition: WinCondition) {
    println!("{} player wins the game with {}!", winner_season, condition);
    println!("Play again soon!");
}
pub(crate) fn round_over(winner_season: Season, condition: WinCondition) {
    println!(
        "{} player wins the round with {}!",
        winner_season, condition
    );
    println!("{} player: when you are ready, press enter.", winner_season);
    Confirm::new("").prompt().expect("Should confirm...");
}

fn show_title(title: &str) {
    let word = format!(" {title} ");

    println!();
    println!("{:=^56}", word);
    println!();
}
fn select_card_from_hand(hand: &Hand) -> usize {
    show_hand(hand);
    let hand_options: Vec<String> = hand.iter().map(|c| c.to_text()).collect();
    let message = "Select a card from your hand";
    let card_option = Select::new(message, hand_options.clone())
        .prompt()
        .expect("Should be a selection");
    hand_options
        .iter()
        .position(|c| *c == card_option)
        .expect("Should be an option")
}
fn select_player_field(seasons: Vec<Season>, card_name: &str) -> Option<usize> {
    let message = format!("On which player's field do you wish to play {}?", card_name);
    let mut options: Vec<String> = seasons.iter().map(|s| format!("{}", s)).collect();
    options.push(String::from("Back"));
    let index = Select::new(&message, options)
        .raw_prompt()
        .expect("Expected a choice")
        .index;
    if index == seasons.len() {
        None
    } else {
        Some(index)
    }
}
fn select_spot_on_field(field: &Field, is_swap: bool, card_name: &str) -> Option<Spot> {
    let all_spots: Vec<Spot> = (0..10usize)
        .map(|i| {
            let row = if i < 5 { Row::Garden } else { Row::Court };
            Spot::new(row, i % 5)
        })
        .collect();

    let spot_options: Vec<(Spot, String)> = if is_swap {
        all_spots
            .iter()
            .filter_map(|spot| field.get(*spot).and_then(|c| Some((*spot, c.to_text()))))
            .collect()
    } else {
        all_spots
            .iter()
            .filter_map(|spot| {
                if field.get(*spot).is_none() {
                    Some((*spot, format!("{}", spot)))
                } else {
                    None
                }
            })
            .collect()
    };
    let spots: Vec<Spot> = spot_options.iter().map(|so| so.0.clone()).collect();
    let mut spot_options: Vec<String> = spot_options.iter().map(|so| so.1.clone()).collect();
    spot_options.push(String::from("Back"));

    show_field(field);
    let message = if is_swap {
        format!("Which card will you swap with {}?", card_name)
    } else {
        format!("Where will you play {}?", card_name)
    };
    let card_index = Select::new(message.as_str(), spot_options)
        .raw_prompt()
        .expect("Should be a card")
        .index;
    if card_index >= spots.len() {
        None
    } else {
        Some(spots[card_index])
    }
}
fn display_row(row: &RowOfCards, f: fn(&Card) -> String) {
    for c in row {
        print!(
            "|{:^10}",
            match c {
                Some(card) => f(card),
                None => String::from(""),
            }
        );
    }
    println!("|");
}
fn display_scores(card: &Card) -> String {
    match (card.garden_score(), card.court_score()) {
        (Score::Value(gs), Score::Value(cs)) => format!("{gs} / {cs}"),
        (Score::Mod(gs), Score::Mod(_)) => match (gs.op, gs.affected) {
            (Op::Add(a), AffectedCards::Row) => format!("Row {a:+}"),
            (Op::Mult(a), AffectedCards::Row) => format!("Row x{a}"),
            _ => panic!("Invalid card"),
        },
        _ => panic!("Invalid card"),
    }
}
fn show_field(field: &Field) {
    println!("+----------+----------+----------+----------+----------+");
    display_row(&field.garden, |card| card.season().to_string());
    display_row(&field.garden, |card| card.rune().to_string());
    display_row(&field.garden, |card| display_scores(card));
    display_row(&field.garden, |card| card.rune().ability().to_string());
    println!("+----------+----------+----------+----------+----------+");
    display_row(&field.court, |card| card.season().to_string());
    display_row(&field.court, |card| card.rune().to_string());
    display_row(&field.court, |card| display_scores(card));
    display_row(&field.court, |card| card.rune().ability().to_string());
    println!("+----------+----------+----------+----------+----------+");
}
fn show_hand(hand: &Hand) {
    show_title("Your  Hand");
    let row_vec: Vec<Option<Card>> = (0..5)
        .map(|i| if i < hand.len() { Some(hand[i]) } else { None })
        .collect();
    let row = [row_vec[0], row_vec[1], row_vec[2], row_vec[3], row_vec[4]];

    println!("+----------+----------+----------+----------+----------+");
    display_row(&row, |card| card.season().to_string());
    display_row(&row, |card| card.rune().to_string());
    display_row(&row, |card| display_scores(card));
    display_row(&row, |card| card.rune().ability().to_string());
    println!("+----------+----------+----------+----------+----------+");

    if hand.len() <= 5 {
        return;
    }
    let row_vec: Vec<Option<Card>> = (5..10)
        .map(|i| if i < hand.len() { Some(hand[i]) } else { None })
        .collect();
    let row = [row_vec[0], row_vec[1], row_vec[2], row_vec[3], row_vec[4]];

    display_row(&row, |card| card.season().to_string());
    display_row(&row, |card| card.rune().to_string());
    display_row(&row, |card| display_scores(card));
    display_row(&row, |card| card.rune().ability().to_string());
    println!("+----------+----------+----------+----------+----------+");
}

#[cfg(test)]
mod test {
    use super::super::field::{Row, Spot};

    use super::*;
    #[test]
    fn test_show_field() {
        let mut field = Field::new();
        field.set(
            Some(Card::load_all("assets/card_list.csv")[56]),
            Spot::new(Row::Court, 0),
        );
        show_field(&field);
    }
    #[test]
    fn test_show_title() {
        show_title(Season::Spring.to_string().as_str());
        show_title(Season::Summer.to_string().as_str());
        show_title(Season::Autumn.to_string().as_str());
        show_title(Season::Winter.to_string().as_str());
        show_title(Season::Ferric.to_string().as_str());
        show_title("This one is longer");
    }
}
