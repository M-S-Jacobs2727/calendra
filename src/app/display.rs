use std::str;

use inquire::{Confirm, InquireError, Select};

use super::{
    card::{score::Score, Card},
    field::{Field, RowOfCards, Spot},
    season::Season,
    win_condition::WinCondition,
};

/// Prompt the user for a 2, 3, or 4-player game
pub(crate) fn get_num_players() -> i32 {
    loop {
        let res = Select::new("Select number of players: ", vec![2, 3, 4]).prompt();
        if let Ok(num_players) = res {
            return num_players;
        }
    }
}

pub(crate) fn show_all_fields(fields: &[&Field]) {
    for field in fields {
        show_field(field);
    }
}
/// Prompt the player to select a card from their hand
pub(crate) fn get_card_choice_from_hand(hand: &[Card]) -> usize {
    show_hand(hand);
    let hand_options: Vec<String> = hand.iter().map(|c| c.to_text()).collect();
    let message = "Select a card from your hand";
    loop {
        match Select::new(message, hand_options.clone()).raw_prompt() {
            Ok(selected_option) => return selected_option.index,
            Err(InquireError::OperationCanceled) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
}
pub(crate) fn select_spot_to_play_card(
    selected_card: &Card,
    valid_spots: &Vec<Spot>,
) -> Option<Spot> {
    let message = format!("Select a spot to play your {}", selected_card);

    match Select::new(&message, valid_spots.clone()).raw_prompt() {
        Ok(selected_spot) => Some(valid_spots[selected_spot.index]),
        Err(InquireError::OperationCanceled) => None,
        Err(_) => panic!(""),
    }
}

pub(crate) fn select_spot_to_swap_card(
    selected_card: &Card,
    valid_spots: Vec<Vec<Spot>>,
    fields: &Vec<&Field>,
    seasons: Vec<Season>,
) -> Option<(usize, Spot)> {
    let field_message = format!("Select a field to play your {} on", selected_card);
    let spot_message = format!("Select a card to swap with your {}", selected_card);
    let available_field_indices: Vec<usize> = (0..valid_spots.len())
        .filter(|i| valid_spots[*i].len() != 0)
        .collect();
    let season_options: Vec<Season> = available_field_indices
        .iter()
        .map(|i| seasons[*i])
        .collect();

    loop {
        let field_index = match Select::new(&field_message, season_options.clone()).raw_prompt() {
            Ok(selected_season) => selected_season.index,
            Err(InquireError::OperationCanceled) => return None,
            Err(_) => panic!("Encountered error"),
        };
        let options: Vec<&Card> = valid_spots[field_index]
            .iter()
            .map(|spot| {
                fields[field_index]
                    .get(*spot)
                    .as_ref()
                    .expect("Should only contain cards")
            })
            .collect();
        let spot_index = match Select::new(&spot_message, options).raw_prompt() {
            Ok(selected_spot) => selected_spot.index,
            Err(InquireError::OperationCanceled) => continue,
            Err(_) => panic!("Encountered error"),
        };
        return Some((field_index, valid_spots[field_index][spot_index]));
    }
}

/// Prompt the winner of a round to choose from the prizes available
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
        .index
}
/// Print a game over screen with the winner and winning condition
pub(crate) fn game_over(winner_season: Season, condition: WinCondition) {
    println!("{} player wins the game with {}!", winner_season, condition);
    println!("Play again soon!");
}
/// Print a round over screen with the winner and winning condition
pub(crate) fn round_over(winner_season: Season, condition: WinCondition) {
    println!(
        "{} player wins the round with {}!",
        winner_season, condition
    );
}
/// Wait for the next player to confirm that they are ready before proceeding
pub(crate) fn wait_for_next_player(season: Season) {
    let message = format!("{} player, press enter to start your turn.", season);
    Confirm::new(&message)
        .with_default(true)
        .prompt()
        .expect("Cancelled");
}

/// Display a title with some fixed styling
fn show_title(title: &str) {
    let word = format!(" {title} ");

    println!();
    println!("{:=^56}", word);
    println!();
}
/// Helper function to display a row of text, one field from each of a row of cards
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
/// Helper function to format the scores into one line
fn display_scores(card: &Card) -> String {
    match (card.garden_score(), card.court_score()) {
        (Score::Value(gs), Score::Value(cs)) => format!("{gs} / {cs}"),
        (Score::Mod(gs), Score::Mod(cs)) => format!("{gs} / {cs}"),
        _ => panic!("Invalid card"),
    }
}
/// Display all ten spots of a given field
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
/// Display up to ten cards in a hand
fn show_hand(hand: &[Card]) {
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
            Some(Card::create_ancient(Season::Autumn)),
            Spot::new(Row::Court, 0),
        );
        show_field(&field);
    }
    #[test]
    fn test_show_title() {
        show_title(&Season::Spring.to_string());
        show_title(&Season::Summer.to_string());
        show_title(&Season::Autumn.to_string());
        show_title(&Season::Winter.to_string());
        show_title(&Season::Ferric.to_string());
        show_title("This one is longer");
    }
}
