pub mod moves;

use std::collections::HashSet;

use itertools::Itertools;
use rand::Rng;

use crate::structs::{board::ISLAND_COORDS, cards::island::IslandCardName};

use self::moves::{Action, MoveType};

use super::{
    board::Board,
    cards::{
        adventurer::{Adventurer, AdventurerCard, AdventurerCardType},
        flood::FloodCard,
        island::{IslandCard, IslandCardState},
        treasure::{SpecialActionType, TreasureCard, TreasureCardType, TreasureType},
        Card, Deck,
    },
};

#[derive(Debug)]
pub struct Game<R: Rng> {
    rng: R,
    treasure_deck: Deck<TreasureCard>,
    treasure_discard_deck: Deck<TreasureCard>,
    flood_deck: Deck<FloodCard>,
    flood_discard_deck: Deck<FloodCard>,
    adventurers: Vec<Adventurer>,
    board: Board,
    water_level: usize,
    captured_treasures: HashSet<TreasureType>,
    used_pilot_move: bool,
    debug: bool,
}

impl<R: Rng> Game<R> {
    pub fn new(mut rng: R, adventurers_count: usize, water_level: usize, debug: bool) -> Game<R> {
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
            debug,
            used_pilot_move: false,
        }
    }

    pub fn play<F: Fn(Action, &Vec<String>) -> usize>(&mut self, player: F) -> (bool, String, u64) {
        let adventurer_types = self
            .adventurers
            .iter()
            .map(|a| *a.get_type())
            .enumerate()
            .collect_vec();

        for (rounds, (index, adventurer_type)) in adventurer_types.iter().cycle().enumerate() {
            // Check for winning
            let fools_landing_pos = self.board.get_location(&IslandCardName::FoolsLanding);
            if self.captured_treasures.len() == 4
                && self
                    .adventurers
                    .iter()
                    .all(|a| a.get_pos() == &fools_landing_pos)
                && self.adventurers.iter().any(|a| {
                    a.get_hand()
                        .contains(&TreasureCard::new(&TreasureCardType::SpecialAction(
                            SpecialActionType::HelicopterLift,
                        )))
                })
            {
                return (true, String::from("You captured all 4 treasures and used a helicopter lift card to get off fools landing, you win!"), rounds as u64);
            }

            if self.debug {
                self.show_state();
            }
            // Play up to 3 moves
            self.initial_actions(adventurer_type, &player, 3);
            self.used_pilot_move = false;

            // Draw 2 Treasure deck cards
            let (card1, card2) = (self.draw_treasure(), self.draw_treasure());
            if self.debug {
                println!(
                    "\n{:?} picked up a {} card and a {} card\n",
                    adventurer_type,
                    &card1.as_string(),
                    &card2.as_string(),
                );
            }
            let mut water_risen = false;
            if card1.get_type() == &TreasureCardType::WaterRise {
                water_risen = true;
                self.water_level += 1;
                self.treasure_discard_deck.insert(card1);
            } else {
                self.adventurers[*index].receive_card(card1);
            }
            if card2.get_type() == &TreasureCardType::WaterRise {
                water_risen = true;
                self.water_level += 1;
                self.treasure_discard_deck.insert(card2);
            } else {
                self.adventurers[*index].receive_card(card2);
            }
            // Water rise?
            if water_risen {
                self.flood_discard_deck.shuffle(&mut self.rng);
                self.flood_deck.stack_front(&mut self.flood_discard_deck);
            }

            // Discard to <= 5 cards
            while self.adventurers[*index].get_card_count() > 5 {
                self.discard_cards(adventurer_type, &player);
            }

            // Draw flood cards
            if self.water_level == 9 {
                return (
                    false,
                    String::from("Water level too high, you lose!"),
                    rounds as u64,
                );
            }
            let flood_card_draw_count = match self.water_level {
                0 | 1 => 2,
                2..=4 => 3,
                5 | 6 => 4,
                7 | 8 => 5,
                _ => unreachable!(),
            };

            for _ in 0..flood_card_draw_count {
                let flood_card = self.flood_deck.pop_next().unwrap();
                let island_card = flood_card.get_type();
                if !self.board.sink(&flood_card.get_type()) {
                    if self.debug {
                        println!("{} becomes flooded!", flood_card.as_string());
                    }
                    // Discard the card if the card sinks
                    self.flood_discard_deck.insert(flood_card);
                } else if self.debug {
                    println!("{} sinks!", flood_card.as_string());
                }
                if self.flood_deck.is_empty() {
                    self.flood_discard_deck.shuffle(&mut self.rng);
                    self.flood_deck.stack(&mut self.flood_discard_deck);
                }

                // Check the game is still playable
                if island_card == IslandCardName::FoolsLanding
                    && self.board.get_by_type(&island_card).state() == &IslandCardState::Sunk
                {
                    return (
                        false,
                        String::from("Fools landing sinks, you lose!"),
                        rounds as u64,
                    );
                }
                for treasure in TreasureType::iter() {
                    if !self.captured_treasures.contains(treasure)
                        && !treasure.retrieved_from().iter().any(|card| {
                            self.board.get_by_type(card).state() != &IslandCardState::Sunk
                        })
                    {
                        return (
                            false,
                            format!("Cannot retrieve {}, you lose!", treasure.get_name()),
                            rounds as u64,
                        );
                    }
                }

                // Move players if needs be
                for i in 0..self.adventurers.len() {
                    let to_move = &self.adventurers[i];
                    if self.board.get_card(to_move.get_pos()).unwrap().state()
                        == &IslandCardState::Sunk
                    {
                        if self.debug {
                            println!(
                                "{:?} must move off {}",
                                to_move.get_type(),
                                self.board.get_card(to_move.get_pos()).unwrap().as_string()
                            );
                        }
                        if self.get_moves(to_move).is_empty() {
                            return (
                                false,
                                format!(
                                    "{:?} cannot move to safety, you lose!",
                                    to_move.get_type()
                                ),
                                rounds as u64,
                            );
                        } else {
                            let adventurer_type = to_move.get_type().to_owned();
                            self.move_adventurer_flood(&adventurer_type, &player);
                        }
                    }
                }
            }
        }
        unreachable!()
    }

    fn draw_treasure(&mut self) -> TreasureCard {
        let card = self.treasure_deck.pop_next().unwrap();
        if self.treasure_deck.is_empty() {
            self.treasure_discard_deck.shuffle(&mut self.rng);
            self.treasure_deck.stack(&mut self.treasure_discard_deck);
        }
        card
    }

    fn show_state(&self) {
        let mut to_show = self.board.show();
        to_show += "\n";
        for adventurer in &self.adventurers {
            to_show += &format!(
                "{:?} ({}): [{}]\n",
                adventurer.get_type(),
                self.board
                    .get_card(adventurer.get_pos())
                    .unwrap()
                    .as_string(),
                adventurer
                    .get_hand()
                    .iter()
                    .map(|c| c.as_string())
                    .join(", ")
            );
        }
        println!("{to_show}");
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
        let adventurer_type = adventurer.get_type();
        let &(x, y) = adventurer.get_pos();
        match (adventurer_type, self.used_pilot_move) {
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
                    || adventurer_type == &AdventurerCardType::Diver)
        })
    }

    pub fn get_moves(&self, adventurer: &Adventurer) -> Vec<(usize, usize)> {
        let adventurer_type = adventurer.get_type();
        let &(x, y) = adventurer.get_pos();
        match (adventurer_type, self.used_pilot_move) {
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
                    || adventurer_type == &AdventurerCardType::Diver)
        })
        .copied()
        .collect()
    }

    pub fn get_shorable_by_adventurer(
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

    fn discard_cards<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
    ) {
        let mut possible_discards = Vec::new();
        for (index, card) in self
            .get_adventurer(adventurer_type)
            .unwrap()
            .get_hand()
            .iter()
            .enumerate()
        {
            let card_type = card.get_type();
            if let &TreasureCardType::SpecialAction(t) = card_type {
                if self.board.has_shorable() || t == SpecialActionType::HelicopterLift {
                    possible_discards.push((
                        (index, true, card_type),
                        format!("Use {} card", card.as_string()),
                    ));
                }
            }
            possible_discards.push((
                (index, false, card_type),
                format!("Discard {} card", card.as_string()),
            ));
        }
        let choice = if possible_discards.len() == 1 {
            0
        } else {
            let action_strings: Vec<_> = possible_discards
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            chooser(Action::DiscardCards, &action_strings)
        };
        let (index, use_special, card_type) = possible_discards[choice].0;
        if use_special {
            if let TreasureCardType::SpecialAction(t) = card_type {
                match t {
                    SpecialActionType::Sandbag => self.sandbag(&chooser),
                    SpecialActionType::HelicopterLift => self.helicopter_lift(&chooser),
                }
            }
        }
        self.get_adventurer_mut(adventurer_type)
            .unwrap()
            .remove_card(index);
    }

    fn initial_actions<F: Fn(Action, &Vec<String>) -> usize>(
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
            intial_choices.push((Action::Move, format!("Move {adventurer_type:?}")));
        }

        if self.can_shore_up(adventurer_type, &adventurer.get_pos()) {
            intial_choices.push((Action::ShoreUp, String::from("Shore up tile")));
        }

        if adventurer.has_cards()
            && (adventurer_type == &AdventurerCardType::Messenger
                || self.adventurers.iter().any(|a| {
                    a.get_type() != adventurer_type && a.get_pos() == adventurer.get_pos()
                }))
        {
            intial_choices.push((Action::GiveCard, String::from("Give card")));
        }

        if let Some(t) = self
            .board
            .get_card(adventurer.get_pos())
            .unwrap()
            .can_retrieve()
        {
            if !self.captured_treasures.contains(&t)
                && adventurer
                    .get_hand()
                    .iter()
                    .filter(|c| c.get_type() == &TreasureCardType::Treasure(t))
                    .count()
                    >= 4
            {
                intial_choices.push((
                    Action::CaptureTreasure(t),
                    format!("Capture {:?}", t.get_name()),
                ));
            }
        }

        if self.adventurers.iter().any(|a| {
            a.get_hand().iter().any(|c| match c.get_type() {
                TreasureCardType::SpecialAction(t) => match t {
                    SpecialActionType::Sandbag => self.board.has_shorable(),
                    SpecialActionType::HelicopterLift => true,
                },
                _ => false,
            })
        }) {
            intial_choices.push((Action::PlayActionCard, String::from("Play action card")));
        }

        intial_choices.push((Action::EndTurn, String::from("End turn")));

        let action_strings: Vec<_> = intial_choices
            .iter()
            .enumerate()
            .map(|(i, (_, s))| (format!("{i}: {s}")))
            .collect();
        let choice = chooser(Action::Initial, &action_strings);

        match intial_choices[choice].0 {
            Action::Move => self.move_adventurer(adventurer_type, chooser, actions_left),
            Action::ShoreUp => self.shore_up(adventurer_type, chooser, actions_left),
            Action::GiveCard => self.give_card(adventurer_type, chooser, actions_left),
            Action::CaptureTreasure(t) => {
                self.capture_treasure(adventurer_type, t, chooser, actions_left)
            }
            Action::PlayActionCard => self.play_action_card(adventurer_type, chooser, actions_left),
            Action::EndTurn => {
                // DO NOTHING
            }
            _ => unreachable!(),
        }
    }

    fn move_adventurer<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        let mut actions = Vec::new();
        // Move
        for pos @ (px, py) in self.get_moves(adventurer) {
            let (x, y) = adventurer.get_pos();
            let pilot_move = x.abs_diff(px) + y.abs_diff(py) > 1;
            if pilot_move {
                actions.push((
                    MoveType::PilotMove(pos),
                    format!(
                        "Use Pilot skill to move to {:?}",
                        self.board.get_card(&pos).unwrap().as_string()
                    ),
                ));
            } else {
                actions.push((
                    MoveType::Move(pos),
                    format!(
                        "Move {:?} to {:?}",
                        adventurer_type,
                        self.board.get_card(&pos).unwrap().as_string()
                    ),
                ));
            }
        }

        // -- Navigator Moves
        if adventurer.get_type() == &AdventurerCardType::Navigator {
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
                            self.board.get_card(&pos).unwrap().as_string()
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
            chooser(Action::Move, &action_strings)
        };

        match actions[choice].0 {
            MoveType::Move(pos) => self
                .get_adventurer_mut(adventurer_type)
                .unwrap()
                .move_to(pos),
            MoveType::NavigatorMove(t, pos) => self.get_adventurer_mut(&t).unwrap().move_to(pos),
            MoveType::PilotMove(pos) => {
                let adventurer = self.get_adventurer_mut(adventurer_type).unwrap();
                adventurer.move_to(pos);
                self.used_pilot_move = true;
            }
        }
        self.initial_actions(adventurer_type, chooser, actions_left - 1);
    }

    fn move_adventurer_flood<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
    ) {
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        let mut actions = Vec::new();
        let &(x, y) = adventurer.get_pos();
        // Move

        let move_options = match adventurer_type {
            AdventurerCardType::Pilot => Vec::from(ISLAND_COORDS),
            AdventurerCardType::Explorer => vec![
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x + 1, y),
                (x - 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
            ],
            _ => vec![(x, y + 1), (x - 1, y), (x + 1, y), (x, y - 1)],
        };
        let mut possible_moves = move_options
            .iter()
            .filter(|&pos @ &(px, py)| {
                pos != &(x, y)
                    && ISLAND_COORDS.contains(pos)
                    && (self.board.get_card(&(px, py)).unwrap().state() != &IslandCardState::Sunk)
            })
            .collect_vec();
        if possible_moves.is_empty() && adventurer_type == &AdventurerCardType::Diver {
            possible_moves.push(
                ISLAND_COORDS
                    .iter()
                    .sorted_by_key(|(px, py)| px.abs_diff(x) + py.abs_diff(y))
                    .find(|pos| self.board.get_card(pos).unwrap().state() != &IslandCardState::Sunk)
                    .unwrap(),
            );
        }
        for pos in possible_moves {
            actions.push((
                pos,
                format!(
                    "Move {:?} to {:?}",
                    adventurer_type,
                    self.board.get_card(pos).unwrap().as_string()
                ),
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
            chooser(Action::Move, &action_strings)
        };
        self.get_adventurer_mut(adventurer_type)
            .unwrap()
            .move_to(*actions[choice].0);
    }

    fn shore_up<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        self.shore_up_sub(adventurer_type, &chooser);
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        if adventurer_type == &AdventurerCardType::Engineer
            && self.can_shore_up(adventurer_type, &adventurer.get_pos())
        {
            let actions = vec![
                (
                    Action::ShoreUp,
                    "Use engineer skill to shore up another tile",
                ),
                (Action::EndAction, "End action"),
            ];
            let action_strings: Vec<_> = actions
                .iter()
                .enumerate()
                .map(|(i, (_, s))| (format!("{i}: {s}")))
                .collect();
            let choice = chooser(Action::ShoreUp, &action_strings);
            match actions[choice].0 {
                Action::ShoreUp => self.shore_up_sub(adventurer_type, &chooser),
                Action::EndAction => {}
                _ => unreachable!(),
            }
        }
        self.initial_actions(adventurer_type, chooser, actions_left - 1);
    }
    fn shore_up_sub<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
    ) {
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        let mut actions = Vec::new();
        for pos in self.get_shorable_by_adventurer(&adventurer.get_type(), &adventurer.get_pos()) {
            actions.push((
                pos,
                format!(
                    "Shore up {:?}",
                    self.board.get_card(&pos).unwrap().as_string()
                ),
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
            chooser(Action::ShoreUp, &action_strings)
        };
        self.board.shore_up(&actions[choice].0);
    }
    fn give_card<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        chooser: F,
        actions_left: usize,
    ) {
        let mut actions = Vec::new();
        let adventurer = self.get_adventurer(adventurer_type).unwrap();
        for a in self.adventurers.iter().filter(|a| {
            a.get_type() != adventurer.get_type()
                && (adventurer_type == &AdventurerCardType::Messenger
                    || a.get_pos() == adventurer.get_pos())
        }) {
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
            chooser(Action::GiveCard, &action_strings)
        };
        let (card_index, reciever_type) = actions[choice].0;
        let card = self
            .get_adventurer_mut(adventurer_type)
            .unwrap()
            .remove_card(card_index);
        self.get_adventurer_mut(&reciever_type)
            .unwrap()
            .receive_card(card);

        self.initial_actions(adventurer_type, chooser, actions_left - 1);
    }

    fn capture_treasure<F: Fn(Action, &Vec<String>) -> usize>(
        &mut self,
        adventurer_type: &AdventurerCardType,
        treasure_type: TreasureType,
        chooser: F,
        actions_left: usize,
    ) {
        let adventurer = self.get_adventurer_mut(adventurer_type).unwrap();
        let mut taken = 0;
        for _ in 0..adventurer.get_card_count() {
            let card = adventurer.remove_card(0);
            if taken < 4 && card.get_type() == &TreasureCardType::Treasure(treasure_type) {
                taken += 1;
            } else {
                adventurer.receive_card(card);
            }
        }
        self.captured_treasures.insert(treasure_type);
        self.initial_actions(adventurer_type, chooser, actions_left - 1);
    }

    fn play_action_card<F: Fn(Action, &Vec<String>) -> usize>(
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
            chooser(Action::PlayActionCard, &action_strings)
        };

        let (adventurer, card_index, card_type) = actions[choice].0;

        self.get_adventurer_mut(&adventurer)
            .unwrap()
            .remove_card(card_index);
        match card_type {
            SpecialActionType::Sandbag => self.sandbag(&chooser),
            SpecialActionType::HelicopterLift => self.helicopter_lift(&chooser),
        }

        self.initial_actions(adventurer_type, chooser, actions_left);
    }

    fn sandbag<F: Fn(Action, &Vec<String>) -> usize>(&mut self, chooser: F) {
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
            chooser(Action::Sandbag, &action_strings)
        };
        self.board
            .shore_up(&self.board.get_location(actions[choice].0));
    }

    fn helicopter_lift<F: Fn(Action, &Vec<String>) -> usize>(&mut self, chooser: F) {
        let mut states = Vec::new();
        for (key, group) in &self
            .adventurers
            .iter()
            .sorted_by_key(|a| a.get_pos())
            .group_by(|a| a.get_pos())
        {
            let dests = ISLAND_COORDS
                .iter()
                .filter(|pos| {
                    self.board.get_card(pos).unwrap().state() != &IslandCardState::Sunk
                        && *pos != key
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
                        self.board.get_card(&pos).unwrap().as_string()
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
            chooser(Action::HelicopterLift, &action_strings)
        };
        let (pos, who) = &actions[choice].0;
        for a in who {
            self.get_adventurer_mut(a).unwrap().move_to(*pos);
        }
    }
}
