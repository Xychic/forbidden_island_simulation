use crate::structs::cards::{
    adventurer::AdventurerCardType,
    treasure::{SpecialActionType, TreasureType},
};

#[derive(Debug)]
pub struct Action {
    action_type: ActionType,
    description: String,
}

impl Action {
    pub fn new(action_type: ActionType, description: String) -> Action {
        Action {
            action_type,
            description,
        }
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn action_type(&self) -> &ActionType {
        &self.action_type
    }
}

#[derive(Debug)]
pub enum ActionType {
    Move((usize, usize)),
    NavigatorMove(AdventurerCardType, (usize, usize)),
    ShoreUp((usize, usize)),
    GiveCard(usize, AdventurerCardType),
    CaptureTreasure(TreasureType),
    PlayActionCard(AdventurerCardType, usize, SpecialActionType),
    EndTurn,
}

impl ActionType {
    pub fn cost(&self) -> usize {
        match self {
            ActionType::PlayActionCard(_, _, _) => 0,
            _ => 1,
        }
    }
}
