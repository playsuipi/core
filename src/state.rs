use crate::action::{Address, Move, Operation};
use crate::card::Card;
use crate::pile::{Pile, PileError};
use crate::rng::{ChaCha20Rng, SliceRandom};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

/// State manipulation errors
#[derive(Debug)]
pub enum StateError {
    InvalidAddress,
    InvalidDiscard,
    InvalidPile(PileError),
    InvalidMove,
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

    /// Collapse all piles to the beginning of the floor array
    pub fn collapse_floor(&mut self) {
        for (i, x) in self
            .floor
            .iter()
            .map(|x| x.take())
            .filter(|x| !x.is_empty())
            .enumerate()
        {
            self.floor[i].replace(x);
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
    pub fn pile(&self, a: Address) -> &RefCell<Pile> {
        match a {
            Address::Hand(i) => &self.player().hand[i as usize],
            Address::Floor(j) => &self.floor[j as usize],
        }
    }

    /// Take the value out of a pile if it is not empty
    pub fn take(&mut self, a: Address) -> Option<Pile> {
        let x = self.pile(a);
        if !x.borrow().is_empty() {
            Some(x.take())
        } else {
            None
        }
    }

    /// Replace the value of an empty pile
    pub fn replace(&mut self, a: Address, p: Pile) -> Result<(), StateError> {
        let x = self.pile(a);
        if x.borrow().is_empty() {
            x.replace(p);
            Ok(())
        } else {
            Err(StateError::PileIsNotEmpty)
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

    /// Attempt to combine the cards from two piles
    pub fn combine<F, G>(
        &mut self,
        reduce: F,
        save: G,
        p: (Address, Address),
    ) -> Result<(), StateError>
    where
        F: FnOnce(&mut Pile, &mut Pile) -> Result<Pile, PileError>,
        G: FnOnce(&mut Self, Pile) -> Result<(), StateError>,
    {
        if let (Some(mut x), Some(mut y)) = (self.take(p.0), self.take(p.1)) {
            match reduce(&mut x, &mut y) {
                Ok(mut z) => {
                    z.owner = self.turn;
                    save(self, z)
                }
                Err(e) => {
                    self.pile(p.0).replace(x);
                    self.pile(p.1).replace(y);
                    Err(StateError::InvalidPile(e))
                }
            }
        } else {
            Err(StateError::InvalidAddress)
        }
    }

    /// Build a pile from two addresses
    pub fn build(&mut self, a: Address, b: Address) -> Result<(), StateError> {
        self.combine(Pile::build, |g, z| Ok(g.replace(a, z)?), (a, b))
    }

    /// Group two piles from two addresses
    pub fn group(&mut self, a: Address, b: Address) -> Result<(), StateError> {
        self.combine(Pile::group, |g, z| Ok(g.replace(a, z)?), (a, b))
    }

    /// Pair a pile with a capturing card
    pub fn pair(&mut self, a: Address, b: Address) -> Result<(), StateError> {
        self.combine(
            Pile::pair,
            |g, z| Ok(g.player().pairs.borrow_mut().push(z)),
            (a, b),
        )
    }

    /// Apply a move to the game state
    pub fn apply(&mut self, m: Move) -> Result<(), StateError> {
        assert_eq!(
            m.actions
                .iter()
                .filter(|a| match a.address {
                    Address::Hand(_) => true,
                    Address::Floor(_) => false,
                })
                .count(),
            1
        ); // More than one hand address in move
        assert!(match m.actions.last().unwrap().address {
            Address::Hand(_) => true,
            Address::Floor(_) => false,
        }); // Hand address is not the last action
        if m.actions.len() == 1 {
            self.discard(m.actions[0].address)?;
        } else {
            let mut builds = vec![];
            for w in m.actions.windows(2).rev() {
                match w[1].operation {
                    Operation::Passive => {
                        builds.push(w[1].address);
                    }
                    Operation::Active => {
                        self.build(w[0].address, w[1].address)?;
                    }
                }
            }
            let destination = m.actions[0].address;
            let pair = m.actions[0].operation == Operation::Active;
            for (i, b) in builds.iter().rev().enumerate() {
                if i == builds.len() - 1 && pair {
                    self.pair(destination, b.to_owned())?;
                } else {
                    self.group(destination, b.to_owned())?;
                }
            }
            assert!(
                pair || self
                    .player()
                    .hand
                    .iter()
                    .position(|x| x.borrow().value == self.pile(destination).borrow().value)
                    .is_some()
            ); // Created a pile with a value player can't pair
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
                    Card::create(Value::Two, Suit::Spades),
                    Card::create(Value::Two, Suit::Diamonds),
                ],
                Value::Two
            )]
        );
    }

    #[test]
    fn test_apply_more_moves() {
        let mut g = setup();

        assert!(g
            .apply(Move::new(vec![Action::new(
                Operation::Passive,
                Address::Hand(0)
            )]))
            .is_ok());

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
    fn test_apply_even_more_moves() {
        let mut g = setup();

        g.discard(Address::Hand(7)).ok();
        g.turn = !g.turn;

        assert!(g
            .apply(Move::new(vec![
                Action::new(Operation::Active, Address::Floor(2)),
                Action::new(Operation::Active, Address::Floor(3)),
                Action::new(Operation::Passive, Address::Floor(1)),
                Action::new(Operation::Active, Address::Floor(4)),
                Action::new(Operation::Passive, Address::Hand(0)),
            ]))
            .is_ok());

        assert_eq!(
            g.dealer.hand,
            [
                empty(),
                single(Value::Four, Suit::Hearts),
                single(Value::Ten, Suit::Spades),
                single(Value::Five, Suit::Spades),
                single(Value::Three, Suit::Diamonds),
                single(Value::Five, Suit::Clubs),
                single(Value::Six, Suit::Spades),
                single(Value::Jack, Suit::Hearts),
            ]
        );

        let mut p = pair(
            vec![
                Card::create(Value::Two, Suit::Spades),
                Card::create(Value::Eight, Suit::Clubs),
                Card::create(Value::Seven, Suit::Diamonds),
                Card::create(Value::Three, Suit::Spades),
                Card::create(Value::Ten, Suit::Diamonds),
            ],
            Value::Ten,
        );
        p.owner = true;
        assert_eq!(g.dealer.pairs.take(), vec![p]);

        assert_eq!(
            g.floor,
            [
                single(Value::Four, Suit::Clubs),
                empty(),
                empty(),
                empty(),
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

    #[test]
    fn test_collapse_floor_method() {
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

        g.collapse_floor();

        assert_eq!(
            g.floor,
            [
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
                empty(),
                empty(),
                empty()
            ]
        );
    }
}
