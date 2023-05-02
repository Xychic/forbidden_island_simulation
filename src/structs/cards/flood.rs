use super::{island::IslandCardName, Card};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloodCard {
    island_card: IslandCardName,
}

#[allow(dead_code)]
impl FloodCard {
    pub fn new(island_card: IslandCardName) -> FloodCard {
        FloodCard { island_card }
    }

    pub fn from_name(name: &IslandCardName) -> FloodCard {
        FloodCard {
            island_card: name.to_owned(),
        }
    }

    pub fn get_type(&self) -> IslandCardName {
        self.island_card
    }
}

impl Card for FloodCard {
    fn get_deck() -> super::Deck<Self> {
        IslandCardName::iter().map(FloodCard::from_name).collect()
    }

    fn as_string(&self) -> String {
        self.get_type().name()
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::cards::Deck;

    use super::*;

    #[test]
    fn test_fn_new() {
        let a = FloodCard {
            island_card: IslandCardName::BreakersBridge,
        };
        let b = FloodCard::new(IslandCardName::BreakersBridge);
        assert_eq!(a, b);
    }

    #[test]
    fn test_fn_from_name() {
        let a = FloodCard {
            island_card: IslandCardName::BreakersBridge,
        };
        let b = FloodCard::from_name(&IslandCardName::BreakersBridge);
        assert_eq!(a, b);
    }

    #[test]
    fn test_fn_get_type() {
        let a = FloodCard::from_name(&IslandCardName::BreakersBridge);
        assert_eq!(a.get_type(), IslandCardName::BreakersBridge);
    }

    #[test]
    fn test_fn_get_deck() {
        let a = Deck {
            cards: vec![
                FloodCard {
                    island_card: IslandCardName::CliffsOfAbandon,
                },
                FloodCard {
                    island_card: IslandCardName::Watchtower,
                },
                FloodCard {
                    island_card: IslandCardName::PhantomRock,
                },
                FloodCard {
                    island_card: IslandCardName::LostLagoon,
                },
                FloodCard {
                    island_card: IslandCardName::MistyMarsh,
                },
                FloodCard {
                    island_card: IslandCardName::TwilightHollow,
                },
                FloodCard {
                    island_card: IslandCardName::CrimsonForest,
                },
                FloodCard {
                    island_card: IslandCardName::Observatory,
                },
                FloodCard {
                    island_card: IslandCardName::BreakersBridge,
                },
                FloodCard {
                    island_card: IslandCardName::DunesOfDeception,
                },
                FloodCard {
                    island_card: IslandCardName::FoolsLanding,
                },
                FloodCard {
                    island_card: IslandCardName::BronzeGate,
                },
                FloodCard {
                    island_card: IslandCardName::GoldGate,
                },
                FloodCard {
                    island_card: IslandCardName::SilverGate,
                },
                FloodCard {
                    island_card: IslandCardName::CopperGate,
                },
                FloodCard {
                    island_card: IslandCardName::IronGate,
                },
                FloodCard {
                    island_card: IslandCardName::TempleOfTheSun,
                },
                FloodCard {
                    island_card: IslandCardName::TempleOfTheMoon,
                },
                FloodCard {
                    island_card: IslandCardName::WhisperingGarden,
                },
                FloodCard {
                    island_card: IslandCardName::HowlingGarden,
                },
                FloodCard {
                    island_card: IslandCardName::CaveOfEmbers,
                },
                FloodCard {
                    island_card: IslandCardName::CaveOfShadows,
                },
                FloodCard {
                    island_card: IslandCardName::TidalPalace,
                },
                FloodCard {
                    island_card: IslandCardName::CoralPalace,
                },
            ],
        };
        let b = FloodCard::get_deck();
        assert_eq!(a, b);
    }

    #[test]
    fn test_fn_as_string() {
        let a = "Bronze Gate";
        let b = FloodCard {
            island_card: IslandCardName::BronzeGate,
        };
        assert_eq!(a, b.as_string());
    }
}
