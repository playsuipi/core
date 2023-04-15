use crate::action::Move;
use crate::card::Card;
use crate::rng::{ChaCha20Rng, SliceRandom};
use crate::sets::{Set, Single};
use std::collections::{HashSet, VecDeque};

pub type Pile = Option<Box<dyn Set>>;

/// A Suipi player's state
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Player {
    pub hand: [Pile; 8],
    // pub pairs: Vec<Pair>,
}

impl Player {
    /// Get a new player from 8 piles
    pub fn new(h: [Pile; 8]) -> Player {
        Player { hand: h }
    }
}

/// A Suipi game's state
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
        for i in 0..8 {
            self.opponent.hand[i] = self.deal_pile();
            self.dealer.hand[i] = self.deal_pile();
        }
    }

    /// Check if the floor contains only unique values
    pub fn unique_floor(&self) -> bool {
        let mut unique = HashSet::new();
        self.floor
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().to_value())
            .all(|v| match v {
                Err(_) => false,
                Ok(v) => unique.insert(v),
            })
    }

    /// Deal four unique cards to the floor
    pub fn deal_floor(&mut self) {
        for i in 0..4 {
            while let None = self.floor[i] {
                self.floor[i] = self.deal_pile();
                if !self.unique_floor() {
                    for c in self.floor[i].as_ref().unwrap().to_cards() {
                        self.deck.push_back(c);
                    }
                    self.floor[i] = None;
                }
            }
        }
    }

    /// Apply a Suipi move to the game state
    pub fn apply(&mut self, m: Move) {
        println!("Action: {:#?}", m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Suit, Value};
    use crate::rng;

    fn single(v: Value, s: Suit) -> Pile {
        Some(Box::new(Single::new(Card::new(v, s))))
    }

    #[test]
    fn test_state_setup() {
        let mut rng = rng::get_seeded_rng([0; 32]);
        let mut g = Game::default();
        g.init_deck();
        g.shuffle_deck(&mut rng);
        g.deal_hands();
        g.deal_floor();

        assert_eq!(
            g.opponent,
            Player::new([
                single(Value::Ace, Suit::Hearts),
                single(Value::King, Suit::Clubs),
                single(Value::Two, Suit::Diamonds),
                single(Value::Ace, Suit::Clubs),
                single(Value::Seven, Suit::Clubs),
                single(Value::Eight, Suit::Spades),
                single(Value::King, Suit::Hearts),
                single(Value::Three, Suit::Spades),
            ])
        );

        assert_eq!(
            g.dealer,
            Player::new([
                single(Value::Ten, Suit::Diamonds),
                single(Value::Four, Suit::Hearts),
                single(Value::Ten, Suit::Spades),
                single(Value::Five, Suit::Spades),
                single(Value::Three, Suit::Diamonds),
                single(Value::Five, Suit::Clubs),
                single(Value::Six, Suit::Spades),
                single(Value::Jack, Suit::Hearts),
            ])
        );

        assert_eq!(
            g.floor,
            [
                single(Value::Four, Suit::Clubs),
                single(Value::Seven, Suit::Diamonds),
                single(Value::Two, Suit::Spades),
                single(Value::Eight, Suit::Clubs),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]
        );
    }
}
