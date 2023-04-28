use super::{treasure::TreasureType, Card, CardType, Deck};
use std::{fmt::Debug, slice::Iter};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IslandCardState {
    Normal,
    Flooded,
    Sunk,
}

impl IslandCardState {
    fn next(&self) -> IslandCardState {
        match self {
            IslandCardState::Normal => IslandCardState::Flooded,
            IslandCardState::Flooded => IslandCardState::Sunk,
            IslandCardState::Sunk => panic!(),
        }
    }

    fn prev(&self) -> IslandCardState {
        match self {
            IslandCardState::Normal => panic!(),
            IslandCardState::Flooded => IslandCardState::Normal,
            IslandCardState::Sunk => IslandCardState::Flooded,
        }
    }

    fn step(&mut self) {
        *self = self.next();
    }

    fn step_back(&mut self) {
        *self = self.prev();
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum IslandCardName {
    CliffsOfAbandon,
    Watchtower,
    PhantomRock,
    LostLagoon,
    MistyMarsh,
    TwilightHollow,
    CrimsonForest,
    Observatory,
    BreakersBridge,
    DunesOfDeception,
    FoolsLanding,
    BronzeGate,
    GoldGate,
    SilverGate,
    CopperGate,
    IronGate,
    TempleOfTheSun,
    TempleOfTheMoon,
    WhisperingGarden,
    HowlingGarden,
    CaveOfEmbers,
    CaveOfShadows,
    TidalPalace,
    CoralPalace,
}
impl IslandCardName {
    pub const fn all() -> [IslandCardName; 24] {
        [
            IslandCardName::CliffsOfAbandon,
            IslandCardName::Watchtower,
            IslandCardName::PhantomRock,
            IslandCardName::LostLagoon,
            IslandCardName::MistyMarsh,
            IslandCardName::TwilightHollow,
            IslandCardName::CrimsonForest,
            IslandCardName::Observatory,
            IslandCardName::BreakersBridge,
            IslandCardName::DunesOfDeception,
            IslandCardName::FoolsLanding,
            IslandCardName::BronzeGate,
            IslandCardName::GoldGate,
            IslandCardName::SilverGate,
            IslandCardName::CopperGate,
            IslandCardName::IronGate,
            IslandCardName::TempleOfTheSun,
            IslandCardName::TempleOfTheMoon,
            IslandCardName::WhisperingGarden,
            IslandCardName::HowlingGarden,
            IslandCardName::CaveOfEmbers,
            IslandCardName::CaveOfShadows,
            IslandCardName::TidalPalace,
            IslandCardName::CoralPalace,
        ]
    }
    pub fn iter() -> Iter<'static, IslandCardName> {
        static ALL_CARD_NAMES: [IslandCardName; 24] = IslandCardName::all();
        ALL_CARD_NAMES.iter()
    }

    pub fn shorthand(&self) -> &'static str {
        match self {
            IslandCardName::CliffsOfAbandon => "C A",
            IslandCardName::Watchtower => " W ",
            IslandCardName::PhantomRock => "P R",
            IslandCardName::LostLagoon => "L L",
            IslandCardName::MistyMarsh => "M M",
            IslandCardName::TwilightHollow => "T H",
            IslandCardName::CrimsonForest => "C F",
            IslandCardName::Observatory => " O ",
            IslandCardName::BreakersBridge => "B B",
            IslandCardName::DunesOfDeception => "D D",
            IslandCardName::FoolsLanding => "F L",
            IslandCardName::BronzeGate => "B G",
            IslandCardName::GoldGate => "G G",
            IslandCardName::SilverGate => "S G",
            IslandCardName::CopperGate => "C G",
            IslandCardName::IronGate => "I G",
            IslandCardName::TempleOfTheSun => "T S",
            IslandCardName::TempleOfTheMoon => "T M",
            IslandCardName::WhisperingGarden => "W G",
            IslandCardName::HowlingGarden => "H G",
            IslandCardName::CaveOfEmbers => "C E",
            IslandCardName::CaveOfShadows => "C S",
            IslandCardName::TidalPalace => "T P",
            IslandCardName::CoralPalace => "C P",
        }
    }
}

impl Debug for IslandCardName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CliffsOfAbandon => write!(f, "Cliffs Of Abandon"),
            Self::Watchtower => write!(f, "Watchtower"),
            Self::PhantomRock => write!(f, "Phantom Rock"),
            Self::LostLagoon => write!(f, "Lost Lagoon"),
            Self::MistyMarsh => write!(f, "Misty Marsh"),
            Self::TwilightHollow => write!(f, "Twilight Hollow"),
            Self::CrimsonForest => write!(f, "Crimson Forest"),
            Self::Observatory => write!(f, "Observatory"),
            Self::BreakersBridge => write!(f, "Breakers Bridge"),
            Self::DunesOfDeception => write!(f, "Dunes Of Deception"),
            Self::FoolsLanding => write!(f, "Fools Landing"),
            Self::BronzeGate => write!(f, "Bronze Gate"),
            Self::GoldGate => write!(f, "Gold Gate"),
            Self::SilverGate => write!(f, "Silver Gate"),
            Self::CopperGate => write!(f, "Copper Gate"),
            Self::IronGate => write!(f, "Iron Gate"),
            Self::TempleOfTheSun => write!(f, "Temple Of The Sun"),
            Self::TempleOfTheMoon => write!(f, "Temple Of The Moon"),
            Self::WhisperingGarden => write!(f, "Whispering Garden"),
            Self::HowlingGarden => write!(f, "Howling Garden"),
            Self::CaveOfEmbers => write!(f, "Cave Of Embers"),
            Self::CaveOfShadows => write!(f, "Cave Of Shadows"),
            Self::TidalPalace => write!(f, "Tidal Palace"),
            Self::CoralPalace => write!(f, "Coral Palace"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct IslandCard {
    state: IslandCardState,
    name: IslandCardName,
}

#[allow(dead_code)]
impl IslandCard {
    pub fn new(name: IslandCardName) -> IslandCard {
        IslandCard {
            state: IslandCardState::Normal,
            name,
        }
    }

    pub fn from_name(&name: &IslandCardName) -> IslandCard {
        IslandCard::new(name)
    }

    pub fn state(&self) -> &IslandCardState {
        &self.state
    }

    pub fn name(&self) -> IslandCardName {
        self.name
    }

    pub fn sink(&mut self) {
        self.state.step();
    }

    pub fn raise(&mut self) {
        assert!(self.state != IslandCardState::Sunk);
        self.state.step_back()
    }

    pub fn tile_str(&self) -> String {
        let (horizontal_sep, vertical_sep) = match self.state {
            IslandCardState::Normal => ("===", "║"),
            IslandCardState::Flooded => ("---", "|"),
            IslandCardState::Sunk => ("   ", " "),
        };
        let shorthand = self.name.shorthand();
        format!("+{horizontal_sep}+\n{vertical_sep}{shorthand}{vertical_sep}\n+{horizontal_sep}+")
    }

    pub fn can_retrieve(&self, treasure: &TreasureType) -> bool {
        match self.name {
            IslandCardName::TempleOfTheSun | IslandCardName::TempleOfTheMoon => {
                treasure == &TreasureType::Earth
            }
            IslandCardName::WhisperingGarden | IslandCardName::HowlingGarden => {
                treasure == &TreasureType::Wind
            }
            IslandCardName::CaveOfEmbers | IslandCardName::CaveOfShadows => {
                treasure == &TreasureType::Fire
            }
            IslandCardName::TidalPalace | IslandCardName::CoralPalace => {
                treasure == &TreasureType::Ocean
            }
            _ => false,
        }
    }
}

impl Card for IslandCard {
    fn card_type() -> CardType {
        CardType::Island
    }

    fn get_deck() -> super::Deck<Self> {
        Deck::from(
            &IslandCardName::iter()
                .map(IslandCard::from_name)
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sink_state() {
        let mut state = IslandCardState::Normal;
        state.step();
        assert_eq!(state, IslandCardState::Flooded);
        state.step();
        assert_eq!(state, IslandCardState::Sunk);
    }

    #[test]
    fn test_sink_next() {
        assert_eq!(IslandCardState::Normal.next(), IslandCardState::Flooded);
        assert_eq!(IslandCardState::Flooded.next(), IslandCardState::Sunk);
    }
}
