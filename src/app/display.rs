use inquire::Select;

use super::{
    card::Card,
    field::{Field, Row, RowOfCards},
    player::Hand,
    AffectedCards, Op, Score, Season, Win,
};

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
pub(crate) fn show_field(field: &Field) {
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
    println!("+----------+----------+----------+----------+----------+");
    display_row(field.row(Row::Garden), |c| c.season().to_string());
    display_row(field.row(Row::Garden), |c| c.rune().to_string());
    display_row(field.row(Row::Garden), |card| display_scores(card));
    display_row(field.row(Row::Garden), |card| {
        card.rune().ability().to_string()
    });
    println!("+----------+----------+----------+----------+----------+");
    display_row(field.row(Row::Court), |c| c.season().to_string());
    display_row(field.row(Row::Court), |c| c.rune().to_string());
    display_row(field.row(Row::Court), |card| display_scores(card));
    display_row(field.row(Row::Court), |card| {
        card.rune().ability().to_string()
    });
    println!("+----------+----------+----------+----------+----------+");
}
pub(crate) fn show_hand(hand: &Hand) {}

pub(crate) fn game_over(win: Win) {
    todo!();
}
pub(crate) fn get_num_players() -> i32 {
    loop {
        let res = Select::new("Select number of players: ", vec![2, 3, 4]).prompt();
        if let Ok(num_players) = res {
            return num_players;
        }
    }
}
#[cfg(test)]
mod test {
    use crate::app::field::Spot;

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
}

pub(crate) fn choose_prize(winner: usize, prizes: Vec<&Card>, seasons: Vec<Season>) -> usize {
    // Display four prizes with winner's on left, other three on right
    // Choice to keep or swap with another player index
    todo!()
}
