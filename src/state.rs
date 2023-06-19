use crate::action::{Address, Move, Operation};
use crate::card::Card;
use crate::pile::Pile;
use crate::rng::{ChaCha20Rng, SliceRandom};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

/// State manipulation errors
pub enum StateError {
    InvalidAddress,
    InvalidBuild,
    InvalidDiscard,
    InvalidGroup,
    InvalidPair,
    FloorIsFull,
    PileIsNotEmpty,
}

/// The state of a player
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Player {
    pub hand: [RefCell<Pile>; 8],
    pub pairs: RefCell<Vec<Pile>>,
}

impl Player {
    /// Get a new player from 8 piles
    pub fn new(hand: [RefCell<Pile>; 8]) -> Player {
        Player {
            hand,
            pairs: RefCell::new(vec![]),
        }
    }

    /// Get the number of cards in a player's hand
    pub fn card_count(&self) -> usize {
        self.hand.iter().filter(|x| !x.borrow().is_empty()).count()
    }
}

/// The state of a game
#[derive(Debug, Default)]
pub struct Game {
    pub deck: VecDeque<Card>,
    pub floor: [RefCell<Pile>; 13],
    pub dealer: Player,
    pub opponent: Player,
    pub turn: bool,
}

impl Game {
    /// Initialize the deck with all 52 cards
    pub fn init_deck(&mut self) {
        for i in 0..52 {
            self.deck.push_back(Card::from(i));
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
            let a = self.deal_pile();
            let b = self.deal_pile();
            self.opponent.hand[i].replace(a);
            self.dealer.hand[i].replace(b);
        }
    }

    /// Check if the floor contains only unique values
    pub fn unique_floor(&self) -> bool {
        let mut unique = HashSet::new();
        self.floor
            .iter()
            .map(|x| x.borrow())
            .filter(|x| !x.is_empty())
            .map(|x| x.value)
            .all(|v| unique.insert(v))
    }

    /// Deal four unique cards to the floor
    pub fn deal_floor(&mut self) {
        for i in 0..4 {
            while self.floor[i].borrow().is_empty() {
                let x = self.deal_pile();
                self.floor[i].replace(x);
                if !self.unique_floor() {
                    for c in self.floor[i].take().cards.to_owned() {
                        self.deck.push_back(c);
                    }
                }
            }
        }
    }

    /// Get the player for the current turn
    pub fn player(&self) -> &Player {
        if self.turn {
            &self.dealer
        } else {
            &self.opponent
        }
    }

    /// Get a pile reference from an address
    pub fn pile(&self, a: Address) -> Option<&RefCell<Pile>> {
        match a {
            Address::Hand(i) => Some(&self.player().hand[i as usize]),
            Address::Floor(j) => Some(&self.floor[j as usize]),
            _ => None,
        }
    }

    /// Take the value out of a pile if it is not empty
    pub fn take(&mut self, a: Address) -> Option<Pile> {
        match self.pile(a) {
            Some(x) => {
                if !x.borrow().is_empty() {
                    Some(x.take())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Replace the value of an empty pile
    pub fn replace(&mut self, a: Address, p: Pile) -> Result<(), StateError> {
        if let Some(x) = self.pile(a) {
            if x.borrow().is_empty() {
                x.replace(p);
                Ok(())
            } else {
                Err(StateError::PileIsNotEmpty)
            }
        } else {
            Err(StateError::InvalidAddress)
        }
    }

    /// Discard a card from your hand
    pub fn discard(&mut self, a: Address) -> Result<(), StateError> {
        match a {
            Address::Hand(_) => {
                if let Some(pile) = self.take(a) {
                    if let Some(j) = self.floor.iter().position(|x| x.borrow().is_empty()) {
                        self.floor[j].replace(pile);
                        if self.unique_floor() {
                            Ok(())
                        } else {
                            let v = self.floor[j].take();
                            self.replace(a, v)?;
                            Err(StateError::InvalidDiscard)
                        }
                    } else {
                        self.replace(a, pile)?;
                        Err(StateError::FloorIsFull)
                    }
                } else {
                    Err(StateError::InvalidDiscard)
                }
            }
            _ => Err(StateError::InvalidAddress),
        }
    }

    /// Build a pile from two addresses
    pub fn build(&mut self, a: Address, b: Address) -> Result<(), StateError> {
        if let (Some(mut x), Some(mut y)) = (self.take(a), self.take(b)) {
            if let Ok(z) = Pile::build(&mut x, &mut y) {
                self.pile(a).unwrap().replace(z);
                Ok(())
            } else {
                self.pile(a).unwrap().replace(x);
                self.pile(b).unwrap().replace(y);
                Err(StateError::InvalidBuild)
            }
        } else {
            Err(StateError::InvalidAddress)
        }
    }

    /// Group two piles from two addresses
    pub fn group(&mut self, a: Address, b: Address) -> Result<(), StateError> {
        if let (Some(mut x), Some(mut y)) = (self.take(a), self.take(b)) {
            if let Ok(z) = Pile::group(&mut x, &mut y) {
                self.pile(a).unwrap().replace(z);
                Ok(())
            } else {
                self.pile(a).unwrap().replace(x);
                self.pile(b).unwrap().replace(y);
                Err(StateError::InvalidGroup)
            }
        } else {
            Err(StateError::InvalidAddress)
        }
    }

    /// Pair a pile with a capturing card
    pub fn pair(&mut self, a: Address, b: Address) -> Result<(), StateError> {
        if let (Some(mut x), Some(mut y)) = (self.take(a), self.take(b)) {
            if let Ok(z) = Pile::pair(&mut x, &mut y) {
                self.player().pairs.borrow_mut().push(z);
                Ok(())
            } else {
                self.pile(a).unwrap().replace(x);
                self.pile(b).unwrap().replace(y);
                Err(StateError::InvalidPair)
            }
        } else {
            Err(StateError::InvalidAddress)
        }
    }

    /// Apply a move to the game state
    pub fn apply(&mut self, m: Move) -> Result<(), StateError> {
        let last_window = (m.actions.len() - 1) / 2;
        for (i, w) in m.actions.windows(2).rev().enumerate() {
            match w[1].operation {
                Operation::Passive => {
                    if i == last_window && w[0].operation == Operation::Active {
                        self.pair(w[1].address, w[0].address)?;
                    } else {
                        self.group(w[0].address, w[1].address)?;
                    }
                }
                Operation::Active => {
                    self.build(w[0].address, w[1].address)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::{Action, Address, Operation};
    use crate::card::{Suit, Value};
    use crate::pile::Mark;
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

    /// Helper for populating a pile with a pair
    fn pair(xs: Vec<Card>, v: Value) -> Pile {
        Pile::new(xs, v as u8, Mark::Pair)
    }

    /// Helper for populating a pile with a group
    fn group(xs: Vec<Card>, v: Value) -> RefCell<Pile> {
        RefCell::new(Pile::new(xs, v as u8, Mark::Group))
    }

    /// Helper for populating a pile with a build
    fn build(xs: Vec<Card>, v: Value) -> RefCell<Pile> {
        RefCell::new(Pile::new(xs, v as u8, Mark::Build))
    }

    /// Helper for populating a pile with a single
    fn single(v: Value, s: Suit) -> RefCell<Pile> {
        RefCell::new(Pile::single(Card::create(v, s)))
    }

    /// Helper for getting an empty pile
    fn empty() -> RefCell<Pile> {
        RefCell::new(Pile::empty())
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

        assert!(g
            .apply(Move::new(vec![
                Action::new(Operation::Active, Address::Floor(2)),
                Action::new(Operation::Passive, Address::Hand(2)),
            ]))
            .is_ok());

        assert_eq!(
            g.opponent.hand,
            [
                single(Value::Ace, Suit::Hearts),
                single(Value::King, Suit::Clubs),
                empty(),
                single(Value::Ace, Suit::Clubs),
                single(Value::Seven, Suit::Clubs),
                single(Value::Eight, Suit::Spades),
                single(Value::King, Suit::Hearts),
                single(Value::Three, Suit::Spades),
            ]
        );

        assert_eq!(
            g.floor,
            [
                single(Value::Four, Suit::Clubs),
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

        assert_eq!(
            g.opponent.pairs.take(),
            vec![pair(
                vec![
                    Card::create(Value::Two, Suit::Diamonds),
                    Card::create(Value::Two, Suit::Spades),
                ],
                Value::Two
            )]
        );
    }

    #[test]
    fn test_build_method() {
        let mut g = setup();

        assert!(g.build(Address::Floor(0), Address::Floor(2)).is_ok());

        assert_eq!(
            g.floor,
            [
                build(
                    vec![
                        Card::create(Value::Four, Suit::Clubs),
                        Card::create(Value::Two, Suit::Spades),
                    ],
                    Value::Six
                ),
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

    #[test]
    fn test_group_method() {
        let mut g = setup();

        assert!(g.build(Address::Floor(0), Address::Hand(7)).is_ok());
        assert!(g.group(Address::Floor(0), Address::Floor(1)).is_ok());

        assert_eq!(
            g.floor,
            [
                group(
                    vec![
                        Card::create(Value::Four, Suit::Clubs),
                        Card::create(Value::Three, Suit::Spades),
                        Card::create(Value::Seven, Suit::Diamonds),
                    ],
                    Value::Seven
                ),
                empty(),
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
    fn test_pair_method() {
        let mut g = setup();

        assert!(g.build(Address::Floor(0), Address::Hand(7)).is_ok());
        assert!(g.group(Address::Floor(0), Address::Floor(1)).is_ok());
        assert!(g.pair(Address::Floor(0), Address::Hand(4)).is_ok());

        assert_eq!(
            g.floor,
            [
                empty(),
                empty(),
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

        assert_eq!(
            g.opponent.pairs.take(),
            vec![pair(
                vec![
                    Card::create(Value::Four, Suit::Clubs),
                    Card::create(Value::Three, Suit::Spades),
                    Card::create(Value::Seven, Suit::Diamonds),
                    Card::create(Value::Seven, Suit::Clubs),
                ],
                Value::Seven
            )]
        );
    }

    #[test]
    fn test_discard_method() {
        let mut g = setup();

        assert!(g.discard(Address::Hand(0)).is_ok());

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
}
