use super::{treasure::TreasureType, Card, Deck};
use std::slice::Iter;

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    pub fn name(&self) -> String {
        String::from(match self {
            IslandCardName::CliffsOfAbandon => "Cliffs Of Abandon",
            IslandCardName::Watchtower => "Watchtower",
            IslandCardName::PhantomRock => "Phantom Rock",
            IslandCardName::LostLagoon => "Lost Lagoon",
            IslandCardName::MistyMarsh => "Misty Marsh",
            IslandCardName::TwilightHollow => "Twilight Hollow",
            IslandCardName::CrimsonForest => "Crimson Forest",
            IslandCardName::Observatory => "Observatory",
            IslandCardName::BreakersBridge => "Breakers Bridge",
            IslandCardName::DunesOfDeception => "Dunes Of Deception",
            IslandCardName::FoolsLanding => "Fools Landing",
            IslandCardName::BronzeGate => "Bronze Gate",
            IslandCardName::GoldGate => "Gold Gate",
            IslandCardName::SilverGate => "Silver Gate",
            IslandCardName::CopperGate => "Copper Gate",
            IslandCardName::IronGate => "Iron Gate",
            IslandCardName::TempleOfTheSun => "Temple Of The Sun",
            IslandCardName::TempleOfTheMoon => "Temple Of The Moon",
            IslandCardName::WhisperingGarden => "Whispering Garden",
            IslandCardName::HowlingGarden => "Howling Garden",
            IslandCardName::CaveOfEmbers => "Cave Of Embers",
            IslandCardName::CaveOfShadows => "Cave Of Shadows",
            IslandCardName::TidalPalace => "Tidal Palace",
            IslandCardName::CoralPalace => "Coral Palace",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IslandCard {
    state: IslandCardState,
    can_retrieve: Option<TreasureType>,
    name: IslandCardName,
}

impl IslandCard {
    pub fn from_name(name: &IslandCardName) -> IslandCard {
        match name {
            IslandCardName::TempleOfTheSun => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Earth),
                name: name.to_owned(),
            },
            IslandCardName::TempleOfTheMoon => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Earth),
                name: name.to_owned(),
            },
            IslandCardName::WhisperingGarden => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Wind),
                name: name.to_owned(),
            },
            IslandCardName::HowlingGarden => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Wind),
                name: name.to_owned(),
            },
            IslandCardName::CaveOfEmbers => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Fire),
                name: name.to_owned(),
            },
            IslandCardName::CaveOfShadows => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Fire),
                name: name.to_owned(),
            },
            IslandCardName::TidalPalace => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Ocean),
                name: name.to_owned(),
            },
            IslandCardName::CoralPalace => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: Some(TreasureType::Ocean),
                name: name.to_owned(),
            },
            _ => IslandCard {
                state: IslandCardState::Normal,
                can_retrieve: None,
                name: name.to_owned(),
            },
        }
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

    pub fn can_retrieve(&self) -> Option<TreasureType> {
        self.can_retrieve
    }
}

impl Card for IslandCard {
    fn get_deck() -> super::Deck<Self> {
        Deck::from(
            &IslandCardName::iter()
                .map(IslandCard::from_name)
                .collect::<Vec<_>>(),
        )
    }

    fn as_string(&self) -> String {
        self.name.name()
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
