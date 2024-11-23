use strum_macros::Display;

use super::score::ScoreModifier;

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
