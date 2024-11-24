use strum_macros::Display;

use super::{Ability, AffectedCards, Op, ScoreModifier};

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
            Rune::Count | Rune::Countess => Ability::ScoreChanging(ScoreModifier {
                op: Op::Add(1),
                affected: AffectedCards::Adjacent,
            }),
        }
    }
}
impl From<&str> for Rune {
    fn from(value: &str) -> Self {
        match value {
            "Ancient" => Rune::Ancient,
            "Archer" => Rune::Archer,
            "Beast" => Rune::Beast,
            "Changeling" => Rune::Changeling,
            "Count" => Rune::Count,
            "Countess" => Rune::Countess,
            "Magician" => Rune::Magician,
            "Mist" => Rune::Mist,
            "Plague" => Rune::Plague,
            "Queen" => Rune::Queen,
            "Warrior" => Rune::Warrior,
            "Weather" => Rune::Weather,
            _ => panic!("Invalid rune: {}", value),
        }
    }
}
