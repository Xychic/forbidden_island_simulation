use crate::structs::cards::{adventurer::AdventurerCardType, treasure::TreasureType};

#[derive(Debug)]
pub enum ActionStage {
    Initial,
    Move,
    ShoreUp,
    GiveCard,
    CaptureTreasure(TreasureType),
    PlayActionCard,
    Sandbag,
    HelicopterLift,
    EndTurn,
    EndAction,
}

#[derive(Debug)]
pub enum MoveType {
    Move((usize, usize)),
    PilotMove((usize, usize)),
    NavigatorMove(AdventurerCardType, (usize, usize)),
}
