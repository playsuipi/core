use crate::action::{Address, Move, Operation};
use crate::card::{Card, Value};
use crate::rng::{ChaCha20Rng, SliceRandom};
use crate::sets::{Build, Set, SetError, Single};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

/// A pile of cards
pub type Pile = RefCell<Option<Box<dyn Set>>>;

/// Helper methods for piles
pub trait PileHelper {
    /// Get a pile from an optional set
    fn new(v: Option<Box<dyn Set>>) -> Pile;

    /// Get an empty pile
    fn empty() -> Pile;

    /// Get a pile from a single card
    fn single(card: Card) -> Pile;

    /// Check if the pile is empty
    fn is_empty(&self) -> bool;

    /// Get all the cards in the pile
    fn to_cards(&self) -> Vec<Card>;

    /// Get the value of the pile
    fn to_value(&self) -> Result<Value, SetError>;
}

impl PileHelper for Pile {
    fn new(v: Option<Box<dyn Set>>) -> Pile {
        RefCell::new(v)
    }

    fn empty() -> Pile {
        Pile::default()
    }

    fn single(card: Card) -> Pile {
        Pile::new(Some(Box::new(Single::new(card))))
    }

    fn is_empty(&self) -> bool {
        self.borrow().is_none()
    }

    fn to_cards(&self) -> Vec<Card> {
        if self.is_empty() {
            vec![]
        } else {
            self.borrow().as_ref().unwrap().to_cards()
        }
    }

    fn to_value(&self) -> Result<Value, SetError> {
        if self.is_empty() {
            Err(SetError::TooFewCards)
        } else {
            self.borrow().as_ref().unwrap().to_value()
        }
    }
}

/// The state of a player
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

    /// Get the cards in the players hand
    pub fn cards(&self) -> Vec<Card> {
        self.hand
            .iter()
            .flat_map(|x| x.to_cards().into_iter())
            .collect()
    }
}

/// The state of a game
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
            None => Pile::empty(),
            Some(card) => Pile::single(card),
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
            .filter(|x| !x.is_empty())
            .map(|x| x.to_value())
            .all(|v| match v {
                Err(_) => false,
                Ok(v) => unique.insert(v),
            })
    }

    /// Deal four unique cards to the floor
    pub fn deal_floor(&mut self) {
        for i in 0..4 {
            while self.floor[i].is_empty() {
                self.floor[i] = self.deal_pile();
                if !self.unique_floor() {
                    for c in self.floor[i].to_cards() {
                        self.deck.push_back(c);
                    }
                    self.floor[i] = Pile::empty();
                }
            }
        }
    }

    /// Get the player for the current turn
    fn player(&self) -> &Player {
        if self.dealer.cards().len() > self.opponent.cards().len() {
            &self.dealer
        } else {
            &self.opponent
        }
    }

    /// Discard to the floor from the player's hand
    fn discard(&mut self, index: usize) {
        if let Some(set) = self.player().hand[index].take() {
            if let Some(j) = self.floor.iter().position(|x| x.is_empty()) {
                self.floor[j].replace(Some(set));
            }
        }
    }

    /// Get a pile from a floor or hand address
    fn to_pile(&self, a: &Address) -> Option<&Pile> {
        match a {
            &Address::Hand(x) => Some(&self.player().hand[x as usize]),
            &Address::Floor(x) => Some(&self.floor[x as usize]),
            _ => None,
        }
    }

    /// Take a pile from a floor or hand address
    fn take_pile(&self, a: &Address) -> Option<Box<dyn Set>> {
        if let Some(x) = self.to_pile(a) {
            x.take()
        } else {
            None
        }
    }

    /// Build two piles into a combined stack
    pub fn build(&mut self, bottom: &Address, top: &Address) {
        let a = self.take_pile(bottom);
        let b = self.take_pile(top);
        if a.is_some() && b.is_some() {
            if let Ok(x) = Build::build(a.unwrap(), b.unwrap()) {
                self.to_pile(bottom).unwrap().replace(Some(Box::new(x)));
            }
        }
    }

    /// Apply a move to the game state
    pub fn apply(&mut self, m: Move) {
        for a in m.actions {
            match a.operation {
                Operation::Passive => match a.address {
                    Address::Hand(x) => self.discard(x.into()),
                    _ => {}
                },
                Operation::Active => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::{Action, Address, Operation};
    use crate::card::{Suit, Value};
    use crate::rng;

    /// Setup an initial game state
    fn setup() -> Game {
        let mut rng = rng::get_seeded_rng([0; 32]);
        let mut g = Game::default();
        g.init_deck();
        g.shuffle_deck(&mut rng);
        g.deal_hands();
        g.deal_floor();
        g
    }

    /// Helper for populating a pile with a single
    fn single(v: Value, s: Suit) -> Pile {
        Pile::single(Card::new(v, s))
    }

    /// Helper for getting an empty pile
    fn empty() -> Pile {
        Pile::empty()
    }

    #[test]
    fn test_state_setup() {
        let g = setup();

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
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty()
            ]
        );
    }

    #[test]
    fn test_apply_move() {
        let mut g = setup();
        g.apply(Move::new(vec![Action::new(
            Operation::Passive,
            Address::Hand(0),
        )]));

        assert_eq!(
            g.opponent,
            Player::new([
                empty(),
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
            g.floor,
            [
                single(Value::Four, Suit::Clubs),
                single(Value::Seven, Suit::Diamonds),
                single(Value::Two, Suit::Spades),
                single(Value::Eight, Suit::Clubs),
                single(Value::Ace, Suit::Hearts),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty()
            ]
        );
    }

    #[test]
    fn test_build_method() {
        let mut g = setup();

        g.build(&Address::Floor(0), &Address::Floor(2));

        assert_eq!(
            g.floor,
            [
                Pile::new(Some(Box::new(Build::new(vec![
                    Card::new(Value::Four, Suit::Clubs),
                    Card::new(Value::Two, Suit::Spades),
                ])))),
                single(Value::Seven, Suit::Diamonds),
                empty(),
                single(Value::Eight, Suit::Clubs),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty()
            ]
        );
    }
}
