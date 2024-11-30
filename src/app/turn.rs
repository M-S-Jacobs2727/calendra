use super::{
    field::{Field, Spot},
    Card,
};

pub(super) fn get_valid_spots_from_card(
    player_index: usize,
    selected_card: &Card,
    fields: &Vec<&Field>,
) -> Vec<Vec<Spot>> {
    if selected_card.rune().ability().is_swap() {
        let mut spots = vec![];
        for i in 0..fields.len() {
            let new_spots: Vec<Spot> = fields[i]
                .iter()
                .enumerate()
                .filter_map(|(i, possible_card)| {
                    if let Some(card) = possible_card {
                        if selected_card.can_swap_with(&card) {
                            return Some(Spot::from_index(i));
                        }
                    }
                    None
                })
                .collect();
            spots.push(new_spots);
        }
        spots
    } else {
        let mut spots = vec![];
        spots.resize(fields.len(), vec![]);
        spots[player_index] = fields[player_index]
            .iter()
            .enumerate()
            .filter_map(|(i, possible_card)| match possible_card {
                Some(_) => None,
                None => Some(Spot::from_index(i)),
            })
            .collect();
        spots
    }
}

pub(super) struct Turn {
    pub player_index: usize,
    pub field_index: usize,
    pub card_index_in_hand: usize,
    pub spot_on_field: Spot,
}
impl Turn {}
