use strum_macros::Display;

#[derive(Display)]
pub(crate) enum Ability {
    /// Can act as a Queen, Beast, Changeling, Count, or Countess to win with the
    /// CountCountess or ThreeInTheCourt win conditions
    Ancient,
    /// Must be played by swapping with a card anywhere on any field.
    /// The swapped card moves to the hand of the player that played this card.
    Swap,
    /// Cannot be swapped with the Changeling or the Plague
    AntiSwap,
    /// Cannot be swapped with the Plague and is not affected by its score modifier
    AntiPlague,
    /// Cannot be affected by the Weather card
    NoWeather,
    /// Adjacent cards have their scores increased by 1
    AdjacentPlusOne,
}
impl Ability {
    pub(crate) fn is_swap(&self) -> bool {
        match self {
            Ability::Swap => true,
            _ => false,
        }
    }
}
