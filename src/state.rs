use crate::action::Action;
use crate::card::{Card, Value};
use crate::rng::{ChaCha20Rng, SliceRandom};
use crate::sets::{Set, SetError, Single};
use std::collections::{HashSet, VecDeque};

pub type Pile = Option<Box<dyn Set>>;

#[derive(Debug, Default)]
pub struct Player {
    pub hand: [Pile; 8],
    // pub pairs: Vec<Pair>,
}

#[derive(Debug, Default)]
pub struct Game {
    pub deck: VecDeque<Card>,
    pub floor: [Pile; 13],
    pub dealer: Player,
    pub opponent: Player,
}

impl Game {
    /// Initialize the deck with all 52 cards
    pub fn init_deck(&mut self) {
        for i in 0..52 {
            self.deck.push_back(Card::from_id(i).unwrap());
        }
    }

    /// Shuffle the deck using the given RNG
    pub fn shuffle_deck(&mut self, rng: &mut ChaCha20Rng) {
        self.deck.make_contiguous().shuffle(rng);
    }

    /// Deal a single card from the deck
    pub fn deal_pile(&mut self) -> Pile {
        match self.deck.pop_front() {
            None => None,
            Some(card) => Some(Box::new(Single::new(card))),
        }
    }

    /// Deal eight cards to each player
    pub fn deal_hands(&mut self) {
        for i in 0..16 {
            if i % 2 == 0 {
                self.opponent.hand[i / 2] = self.deal_pile();
            } else {
                self.dealer.hand[(i - 1) / 2] = self.deal_pile();
            }
        }
    }

    /// Check if the floor contains only unique values
    pub fn unique_floor(&self) -> Result<bool, SetError> {
        let mut unique = HashSet::new();
        match self
            .floor
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().to_value())
            .collect::<Result<Vec<Value>, SetError>>()
        {
            Err(e) => Err(e),
            Ok(values) => Ok(values.iter().all(|v| unique.insert(v))),
        }
    }

    /// Deal four unique cards to the floor
    pub fn deal_floor(&mut self) {
        for i in 0..4 {
            while let None = self.floor[i] {
                self.floor[i] = self.deal_pile();
                if let Ok(false) = self.unique_floor() {
                    for c in self.floor[i].as_ref().unwrap().to_cards() {
                        self.deck.push_back(c);
                    }
                    self.floor[i] = None;
                }
            }
        }
    }

    /// Play an action and save the resulting state
    pub fn play(&mut self, x: Action) {
        println!("Action: {:#?}", x);
    }
}
