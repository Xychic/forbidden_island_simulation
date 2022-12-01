use super::{treasure::TreasureType, Card, CardType};
use std::slice::Iter;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum IslandCardState {
    Normal,
    Flooded,
    Sunk,
}

#[derive(Debug, Clone)]
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
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IslandCard {
    state: IslandCardState,
    can_retrieve: Option<TreasureType>,
    name: IslandCardName,
}

#[allow(dead_code)]
impl IslandCard {
    pub fn new(can_retrieve: Option<TreasureType>, name: IslandCardName) -> IslandCard {
        IslandCard {
            state: IslandCardState::Normal,
            can_retrieve: can_retrieve,
            name,
        }
    }

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
}

impl Card for IslandCard {
    fn card_type() -> CardType {
        CardType::Island
    }
}
