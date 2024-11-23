use strum_macros::Display;

use super::{
    score::{AffectedCards, Op, ScoreModifier},
    Ability, Score,
};
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
    pub fn ability(&self) -> Ability {
        match self {
            Rune::Ancient => Ability::Ancient,
            Rune::Changeling | Rune::Mist | Rune::Plague => Ability::Swap,
            Rune::Archer | Rune::Warrior => Ability::AntiSwap,
            Rune::Queen | Rune::Magician => Ability::AntiPlague,
            Rune::Beast => Ability::NoWeather,
            Rune::Count | Rune::Countess => Ability::ScoreChanging(ScoreModifier {
                op: Op::Add(1),
                affected: AffectedCards::Adjacent,
            }),
            Rune::Weather => Ability::ScoreChanging(ScoreModifier {
                op: Op::Mult(2),
                affected: AffectedCards::Row,
            }),
        }
    }
}
