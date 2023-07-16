use playsuipi_core::action::Annotation;
use playsuipi_core::card::{Card, Suit, Value};
use playsuipi_core::pile::{Mark, Pile};
use playsuipi_core::rng;
use playsuipi_core::state::{Game, StateError};
use std::cell::RefCell;

/// Setup an initial game state
pub fn setup_default() -> Game {
    setup([0; 32])
}

/// Setup an initial game state for the given seed
pub fn setup(seed: [u8; 32]) -> Game {
    let mut rng = rng::get_seeded_rng(seed);
    let mut g = Game::default();
    g.init_deck();
    g.shuffle_deck(&mut rng);
    g.deal_hands();
    g.deal_floor();
    g
}

/// Apply a move to the game from a string annotation
pub fn apply(g: &mut Game, x: &str) -> Result<(), StateError> {
    match Annotation::new(String::from(x)).to_move() {
        Ok(m) => g.apply(m),
        Err(_) => Err(StateError::InvalidMove)
    }
}

/// Helper for populating a pile with a pair
pub fn pair(xs: Vec<Card>, v: Value) -> Pile {
    Pile::new(xs, v as u8, Mark::Pair)
}

/// Helper for populating a pile with a group
pub fn group(xs: Vec<Card>, v: Value) -> RefCell<Pile> {
    RefCell::new(Pile::new(xs, v as u8, Mark::Group))
}

/// Helper for populating a pile with a build
pub fn build(xs: Vec<Card>, v: Value) -> RefCell<Pile> {
    RefCell::new(Pile::new(xs, v as u8, Mark::Build))
}

/// Helper for populating a pile with a single
pub fn single(v: Value, s: Suit) -> RefCell<Pile> {
    RefCell::new(Pile::single(Card::create(v, s)))
}

/// Helper for getting an empty pile
pub fn empty() -> RefCell<Pile> {
    RefCell::new(Pile::empty())
}
