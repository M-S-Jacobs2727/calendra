use strum_macros::Display;

#[derive(Clone, Copy, Display, PartialEq, Debug)]
pub(crate) enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
    Ferric,
}
