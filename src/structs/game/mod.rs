pub mod moves;

use std::collections::HashSet;

use itertools::Itertools;
use rand::Rng;

use crate::structs::board::ISLAND_COORDS;

use self::moves::{ActionStage, MoveType};

use super::{
    board::Board,
    cards::{
        adventurer::{AdventurerCard, AdventurerCardType},
        flood::FloodCard,
        island::{IslandCard, IslandCardState},
        treasure::{SpecialActionType, TreasureCard, TreasureCardType, TreasureType},
        Card, Deck,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adventurer {
    card: AdventurerCardType,
    pub pos: (usize, usize),
    hand: Deck<TreasureCard>,
    used_pilot_move: bool,
}

impl Adventurer {
    pub fn new(card: AdventurerCard, board: &Board) -> Adventurer {
        let start_card = card.get_start_card();
        Adventurer {
            card: card.get_type(),
            pos: board.get_location(&start_card),
            hand: Deck::with_capacity(10),
            used_pilot_move: false,
        }
    }

    pub fn get_type(&self) -> &AdventurerCardType {
        &self.card
    }

    pub fn get_card_count(&self) -> usize {
        self.hand.len()
    }

    pub fn get_hand(&self) -> &Deck<TreasureCard> {
        &self.hand
    }

    pub fn remove_card(&mut self, index: usize) -> TreasureCard {
        self.hand.pop_card(index).unwrap()
    }

    pub fn _find_card(&self, card_type: &TreasureCardType) -> Option<usize> {
        for (i, card) in self.hand.iter().enumerate() {
            if card.get_type() == card_type {
                return Some(i);
            }
        }
        None
    }

    pub fn receive_card(&mut self, card: TreasureCard) {
        self.hand.insert(card);
    }
}

#[derive(Debug)]
pub struct Game<R: Rng> {
    pub rng: R,
    pub treasure_deck: Deck<TreasureCard>,
    pub treasure_discard_deck: Deck<TreasureCard>,
    pub flood_deck: Deck<FloodCard>,
    pub flood_discard_deck: Deck<FloodCard>,
    pub adventurers: Vec<Adventurer>, // TODO Order of hashmap not consistent
    pub board: Board,
    pub water_level: usize,
    pub captured_treasures: HashSet<TreasureType>,
}

impl<R: Rng> Game<R> {
    pub fn new(mut rng: R, adventurers_count: usize, water_level: usize) -> Game<R> {
        let mut island_deck = IslandCard::get_deck();
        let mut adventurer_deck = AdventurerCard::get_deck();

        let mut treasure_deck = TreasureCard::get_deck();
        let mut treasure_discard_deck = Deck::with_capacity(28);
        let mut flood_deck = FloodCard::get_deck();
        let mut flood_discard_deck = Deck::with_capacity(24);

        // Shuffle the decks
        island_deck.shuffle(&mut rng);
        treasure_deck.shuffle(&mut rng);
        flood_deck.shuffle(&mut rng);
        adventurer_deck.shuffle(&mut rng);

        // Setup 1 - Create the island
        let mut board = Board::new(&mut island_deck);

        // Setup 4 - The Island Starts to sink
        for _ in 0..6 {
            let to_sink = flood_deck.pop_next().unwrap();
            board.sink(&to_sink.get_type());
            flood_discard_deck.insert(to_sink);
        }

        // Setup 5 - The Adventurers Appear
        let mut adventurers: Vec<_> = (0..adventurers_count)
            .map(|_| {
                let adventurer = adventurer_deck.pop_next().unwrap();
                Adventurer::new(adventurer, &board)
            })
            .collect();

        // Setup 6 - Hand out Treasure Deck Cards
        for _ in 0..2 {
            for adventurer in adventurers.iter_mut() {
                // adventurer.receive_card()
                loop {
                    let card = treasure_deck.pop_next().unwrap();
                    if card.get_type() == &TreasureCardType::WaterRise {
                        treasure_discard_deck.insert(card);
                    } else {
                        adventurer.receive_card(card);
                        break;
                    }
                }
            }
        }

        treasure_deck.stack(&mut treasure_discard_deck);
        treasure_deck.shuffle(&mut rng);

        Game {
            rng,
            treasure_deck,
            treasure_discard_deck,
            flood_deck,
            flood_discard_deck,
            adventurers,
            board,
            water_level,
            captured_treasures: HashSet::with_capacity(4),
        }
    }

    pub fn get_adventurer(&self, adventurer_type: &AdventurerCardType) -> Option<&Adventurer> {
        self.adventurers
            .iter()
            .find(|&a| a.get_type() == adventurer_type)
    }

    pub fn get_adventurer_mut(
        &mut self,
        adventurer_type: &AdventurerCardType,
    ) -> Option<&mut Adventurer> {
        self.adventurers
            .iter_mut()
            .find(|a| a.get_type() == adventurer_type)
    }

    fn can_move(&self, adventurer: &Adventurer) -> bool {
        let adventurer_type = adventurer.card;
        let (x, y) = adventurer.pos;
        match (adventurer_type, adventurer.used_pilot_move) {
            (AdventurerCardType::Explorer, _) => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            (AdventurerCardType::Pilot, false) => Vec::from(ISLAND_COORDS),
            _ => vec![(x, y + 1), (x - 1, y), (x + 1, y), (x, y - 1)],
        }
        .iter()
        .any(|&pos @ (px, py)| {
            pos != (x, y)
                && ISLAND_COORDS.contains(&pos)
                && (self.board.get_card(&(px, py)).unwrap().state() != &IslandCardState::Sunk
                    || adventurer_type == AdventurerCardType::Diver)
        })
    }

    pub fn get_moves(&self, adventurer: &Adventurer) -> Vec<(usize, usize)> {
        let adventurer_type = adventurer.card;
        let (x, y) = adventurer.pos;
        match (adventurer_type, adventurer.used_pilot_move) {
            (AdventurerCardType::Explorer, _) => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            (AdventurerCardType::Pilot, false) => Vec::from(ISLAND_COORDS),
            _ => vec![(x, y + 1), (x - 1, y), (x + 1, y), (x, y - 1)],
        }
        .iter()
        .filter(|&pos @ &(px, py)| {
            pos != &(x, y)
                && ISLAND_COORDS.contains(pos)
                && (self.board.get_card(&(px, py)).unwrap().state() != &IslandCardState::Sunk
                    || adventurer_type == AdventurerCardType::Diver)
        })
        .copied()
        .collect()
    }

    pub fn get_shorable(
        &self,
        adventurer_type: &AdventurerCardType,
        &(x, y): &(usize, usize),
    ) -> Vec<(usize, usize)> {
        match adventurer_type {
            AdventurerCardType::Explorer => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            _ => vec![(x, y + 1), (x - 1, y), (x, y), (x + 1, y), (x, y - 1)],
        }
        .iter()
        .filter(|&pos| {
            ISLAND_COORDS.contains(pos)
                && self.board.get_card(pos).unwrap().state() == &IslandCardState::Flooded
        })
        .copied()
        .collect()
    }

    fn can_shore_up(&self, adventurer_type: &AdventurerCardType, &(x, y): &(usize, usize)) -> bool {
        match adventurer_type {
            AdventurerCardType::Explorer => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            _ => vec![(x, y + 1), (x - 1, y), (x, y), (x + 1, y), (x, y - 1)],
        }
        .iter()
        .any(|pos| {
            ISLAND_COORDS.contains(pos)
                && self.board.get_card(pos).unwrap().state() == &IslandCardState::Flooded
        })
    }

    pub fn intial_actions<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        if actions_left == 0 {
            return;
        }

        let mut intial_choices = Vec::with_capacity(6);
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        if self.can_move(adventurer) {
            intial_choices.push((ActionStage::Move, format!("Move {adventurer_type:?}")));
        }

        if self.can_shore_up(adventurer_type, &adventurer.pos) {
            intial_choices.push((ActionStage::ShoreUp, String::from("Shore up tile")));
        }

        if adventurer.get_card_count() != 0
            && (adventurer_type == &AdventurerCardType::Messenger
                || self
                    .adventurers
                    .iter()
                    .any(|a| a.get_type() != adventurer_type && a.pos == adventurer.pos))
        {
            intial_choices.push((ActionStage::GiveCard, String::from("Give card")));
        }

        if let Some(t) = self.board.get_card(&adventurer.pos).unwrap().can_retrieve() {
            if !self.captured_treasures.contains(&t)
                && adventurer
                    .hand
                    .iter()
                    .filter(|c| c.get_type() == &TreasureCardType::Treasure(t))
                    .count()
                    >= 4
            {
                intial_choices.push((
                    ActionStage::CaptureTreasure(t),
                    format!("Capture {:?}", t.get_name()),
                ));
            }
        }

        if self.adventurers.iter().any(|a| {
            a.get_hand()
                .iter()
                .any(|c| matches!(c.get_type(), TreasureCardType::SpecialAction(_)))
        }) {
            intial_choices.push((
                ActionStage::PlayActionCard,
                String::from("Play action card"),
            ));
        }

        intial_choices.push((ActionStage::EndTurn, String::from("End turn")));

        let action_strings: Vec<_> = intial_choices
            .iter()
            .enumerate()
            .map(|(i, (_, s))| (format!("{i}: {s}")))
            .collect();
        let choice = chooser(ActionStage::Initial, &action_strings);
        match intial_choices[choice].0 {
            ActionStage::Move => self.handle_move(adventurer_type, chooser, actions_left),
            ActionStage::ShoreUp => self.handle_shore_up(adventurer_type, chooser, actions_left),
            ActionStage::GiveCard => self.handle_give_card(adventurer_type, chooser, actions_left),
            ActionStage::CaptureTreasure(t) => {
                self.handle_capture_treasure(adventurer_type, t, chooser, actions_left)
            }
            ActionStage::PlayActionCard => {
                self.play_action_card(adventurer_type, chooser, actions_left)
            }
            ActionStage::EndTurn => {
                // DO NOTHING
            }
            _ => unreachable!(),
        }
    }

    fn handle_move<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        let mut actions = Vec::new();
        // Move
        for pos @ (px, py) in self.get_moves(adventurer) {
            let (x, y) = adventurer.pos;
            let pilot_move = x.abs_diff(px) + y.abs_diff(py) > 1;
            if pilot_move {
                actions.push((
                    MoveType::PilotMove(pos),
                    format!(
                        "Use Pilot skill to move to {:?}",
                        self.board.get_card(&pos).unwrap().name()
                    ),
                ));
            } else {
                actions.push((
                    MoveType::Move(pos),
                    format!("Move to {:?}", self.board.get_card(&pos).unwrap().name()),
                ));
            }
        }

        // -- Navigator Moves
        if adventurer.card == AdventurerCardType::Navigator {
            for t in self
                .adventurers
                .iter()
                .map(|a| a.get_type())
                .filter(|&t| t != &AdventurerCardType::Navigator)
            {
                for pos in self.get_moves(self.get_adventurer(t).unwrap()) {
                    actions.push((
                        MoveType::NavigatorMove(t.to_owned(), pos),
                        format!(
                            "Move {:?} to {:?}",
                            t,
                            self.board.get_card(&pos).unwrap().name()
                        ),
                    ));
                }
            }
        }

        let choice = if actions.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(ActionStage::Move, &action_strings)
        };

        match actions[choice].0 {
            MoveType::Move(pos) => self.get_adventurer_mut(adventurer_type).unwrap().pos = pos,
            MoveType::NavigatorMove(t, pos) => self.get_adventurer_mut(&t).unwrap().pos = pos,
            MoveType::PilotMove(pos) => {
                let adventurer = self.get_adventurer_mut(adventurer_type).unwrap();
                adventurer.pos = pos;
                adventurer.used_pilot_move = true;
            }
        }
        self.intial_actions(adventurer_type, chooser, actions_left - 1);
    }

    fn handle_shore_up<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        self.shore_up_sub(adventurer_type, &chooser);
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        if adventurer_type == &AdventurerCardType::Engineer
            && self.can_shore_up(adventurer_type, &adventurer.pos)
        {
            let actions = vec![
                (
                    ActionStage::ShoreUp,
                    "Use engineer skill to shore up another tile",
                ),
                (ActionStage::EndAction, "End action"),
            ];
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            let choice = chooser(ActionStage::ShoreUp, &action_strings);
            match actions[choice].0 {
                ActionStage::ShoreUp => self.shore_up_sub(adventurer_type, &chooser),
                ActionStage::EndAction => {}
                _ => unreachable!(),
            }
        }
        self.intial_actions(adventurer_type, chooser, actions_left - 1);
    }
    fn shore_up_sub<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
    ) {
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        let mut actions = Vec::new();
        for pos in self.get_shorable(&adventurer.card, &adventurer.pos) {
            actions.push((
                pos,
                format!("Shore up {:?}", self.board.get_card(&pos).unwrap().name()),
            ));
        }

        let choice = if actions.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(ActionStage::Move, &action_strings)
        };
        self.board.shore_up(&actions[choice].0);
    }
    fn handle_give_card<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        let mut actions = Vec::new();
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        for a in self
            .adventurers
            .iter()
            .filter(|a| a.card != adventurer.card && a.pos == adventurer.pos)
        {
            for (i, c) in adventurer
                .get_hand()
                .iter()
                .enumerate()
                .unique_by(|&(_, x)| x)
            {
                actions.push((
                    (i, *a.get_type()),
                    format!("Give {:?} card to {:?}", c.get_type(), a.get_type()),
                ));
            }
        }
        let choice = if actions.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(ActionStage::Move, &action_strings)
        };
        let (card_index, reciever_type) = actions[choice].0;
        let card = self
            .get_adventurer_mut(adventurer_type)
            .unwrap()
            .remove_card(card_index);
        self.get_adventurer_mut(&reciever_type)
            .unwrap()
            .receive_card(card);

        self.intial_actions(adventurer_type, chooser, actions_left - 1);
    }

    fn handle_capture_treasure<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        treausre_type: TreasureType,
        chooser: F,
        actions_left: usize,
    ) {
        let adventurer = self.get_adventurer_mut(adventurer_type).unwrap();
        let mut new_deck = Deck::with_capacity(1);
        let mut taken = 0;
        for card in adventurer.hand.iter() {
            if taken < 4 && card.get_type() == &TreasureCardType::Treasure(treausre_type) {
                taken += 1;
            } else {
                new_deck.insert(card.to_owned());
            }
        }
        adventurer.hand = new_deck;
        self.captured_treasures.insert(treausre_type);
        self.intial_actions(adventurer_type, chooser, actions_left - 1);
    }

    fn play_action_card<F: Fn(ActionStage, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        let mut actions = Vec::new();
        for a in &self.adventurers {
            for (i, c) in a
                .get_hand()
                .iter()
                .enumerate()
                .filter_map(|(i, c)| match c.get_type() {
                    TreasureCardType::SpecialAction(t) => match t {
                        SpecialActionType::Sandbag => {
                            if self.board.has_shorable() {
                                Some((i, t))
                            } else {
                                None
                            }
                        }
                        SpecialActionType::HelicopterLift => Some((i, t)),
                    },
                    _ => None,
                })
            {
                let t = a.get_type();
                actions.push(((*t, i, *c), format!("Play {t:?}'s {c:?} card")));
            }
        }

        let choice = if actions.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(ActionStage::Move, &action_strings)
        };

        let (adventurer, card_index, card_type) = actions[choice].0;

        self.get_adventurer_mut(&adventurer)
            .unwrap()
            .remove_card(card_index);
        match card_type {
            SpecialActionType::Sandbag => self.sandbag(&chooser),
            SpecialActionType::HelicopterLift => self.helicopter_lift(&chooser),
        }

        self.intial_actions(adventurer_type, chooser, actions_left);
    }

    fn sandbag<F: Fn(ActionStage, &Vec<String>) -> usize>(&mut self, chooser: F) {
        let shorable = self.board.get_shorable();
        let actions: Vec<_> = shorable
            .iter()
            .map(|c| (c, format!("Shore up {c:?}")))
            .collect();
        let choice = if actions.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(ActionStage::Sandbag, &action_strings)
        };
        self.board
            .shore_up(&self.board.get_location(actions[choice].0));
    }

    fn helicopter_lift<F: Fn(ActionStage, &Vec<String>) -> usize>(&mut self, chooser: F) {
        let mut states = Vec::new();
        for (key, group) in &self
            .adventurers
            .iter()
            .sorted_by_key(|a| a.pos)
            .group_by(|a| a.pos)
        {
            let dests = ISLAND_COORDS
                .iter()
                .filter(|pos| {
                    self.board.get_card(pos).unwrap().state() != &IslandCardState::Flooded
                        && **pos != key
                })
                .collect_vec();
            let adventurers = group.map(|a| *a.get_type()).collect_vec();
            for n in 1..=adventurers.len() {
                states.push((n, dests.clone(), adventurers.clone()));
            }
        }

        let actions = states
            .iter()
            .flat_map(|(n, dests, adventurers)| {
                dests
                    .iter()
                    .cartesian_product(adventurers.iter().combinations(*n))
            })
            .map(|(&&pos, who)| {
                (
                    (pos, who.to_owned()),
                    format!(
                        "Move {} to {:?}",
                        who.iter().map(|a| format!("{a:?}")).join(", "),
                        self.board.get_card(&pos).unwrap().name()
                    ),
                )
            })
            .collect_vec();

        let choice = if actions.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(ActionStage::HelicopterLift, &action_strings)
        };
        let (pos, who) = &actions[choice].0;
        for a in who {
            self.get_adventurer_mut(a).unwrap().pos = *pos;
        }
    }
}
