use playsuipi_core::action::Annotation;
use playsuipi_core::card::{Card, Suit, Value};
use playsuipi_core::pile::{Mark, Pile};
use playsuipi_core::rng::Rng;
use playsuipi_core::state::{State, StateError};

/// A pile owner
pub enum Owner {
    Opponent,
    Dealer,
}

impl Into<bool> for Owner {
    fn into(self) -> bool {
        match self {
            Owner::Opponent => false,
            Owner::Dealer => true,
        }
    }
}

/// Setup an initial game state
pub fn setup_default() -> State {
    setup([0; 32])
}

/// Setup an initial game state for the given seed
pub fn setup(seed: [u8; 32]) -> State {
    let mut rng = Rng::from_seed(seed);
    let mut g = State::default();
    g.init_deck();
    g.shuffle_deck(rng.borrow_mut());
    g.deal_hands();
    g.deal_floor();
    g
}

/// Apply a move to the game from a string annotation
pub fn apply(g: &mut State, x: &str) -> Result<(), StateError> {
    match Annotation::new(String::from(x)).to_move() {
        Ok(m) => g.apply(m),
        Err(_) => Err(StateError::InvalidInput),
    }
}

/// Apply a set of moves to initialize game state
pub fn apply_moves(g: &mut State, xs: Vec<&str>) {
    for x in xs {
        assert!(apply(g, x).is_ok());
        g.collapse_floor();
        g.turn = !g.turn;
    }
}

/// Helper for populating a pile with a pair
pub fn pair(xs: Vec<Card>, v: Value, o: Owner) -> Pile {
    let mut p = Pile::new(xs, v as u8, Mark::Pair);
    p.owner = o.into();
    p
}

/// Helper for populating a pile with a group
pub fn group(xs: Vec<Card>, v: Value) -> Pile {
    Pile::new(xs, v as u8, Mark::Group)
}

/// Helper for populating a pile with a build
pub fn build(xs: Vec<Card>, v: Value) -> Pile {
    Pile::new(xs, v as u8, Mark::Build)
}

/// Helper for populating a pile with a single
pub fn single(v: Value, s: Suit) -> Pile {
    Pile::single(Card::create(v, s))
}

/// Helper for getting an empty pile
pub fn empty() -> Pile {
    Pile::empty()
}
