use strum_macros::Display;

use super::Ability;

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub(crate) enum Rune {
    Ancient,
    Archer,
    Beast,
    Changeling,
    Count,
    Countess,
    Magician,
    Mist,
    Plague,
    Queen,
    Warrior,
    Weather,
}
impl Rune {
    pub(crate) fn ability(&self) -> Ability {
        match self {
            Rune::Ancient => Ability::Ancient,
            Rune::Changeling | Rune::Mist | Rune::Plague => Ability::Swap,
            Rune::Archer | Rune::Warrior => Ability::AntiSwap,
            Rune::Queen | Rune::Magician | Rune::Weather => Ability::AntiPlague,
            Rune::Beast => Ability::NoWeather,
            Rune::Count | Rune::Countess => Ability::AdjacentPlusOne,
        }
    }
}
