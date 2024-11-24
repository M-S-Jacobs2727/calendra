use strum_macros::Display;

use super::ScoreModifier;

#[derive(Display)]
pub(crate) enum Ability {
    Ancient,
    Swap,
    AntiSwap,
    AntiPlague,
    NoWeather,
    #[strum(to_string = "{0}")]
    ScoreChanging(ScoreModifier),
}
impl Ability {
    pub(crate) fn is_swap(&self) -> bool {
        match self {
            Ability::Swap => true,
            _ => false,
        }
    }
}
