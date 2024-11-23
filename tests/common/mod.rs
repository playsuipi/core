use playsuipi_core::api;
use playsuipi_core::api::Scorecard;
use playsuipi_core::card::{Card, Suit, Value};
use playsuipi_core::game::Game;
use playsuipi_core::pile::{Mark, Pile};
use std::ffi::{CStr, CString};

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
pub fn setup_default() -> Box<Game> {
    setup([0; 32])
}

/// Setup an initial game state for the given seed
pub fn setup(seed: [u8; 32]) -> Box<Game> {
    unsafe { api::new_game(&seed) }
}

/// Read the current floor state
pub fn read_floor(g: &Box<Game>) -> Vec<Pile> {
    api::read_floor(g).iter().map(|&c| c.into()).collect()
}

/// Read the current player hand states
pub fn read_hands(g: &Box<Game>) -> Vec<Card> {
    api::read_hands(g).iter().map(|&c| Card::from(c)).collect()
}

/// Read the game scorecards
pub fn get_scores(g: &Box<Game>) -> Box<[Scorecard; 4]> {
    api::get_scores(g)
}

/// Get a game scorecard
pub fn scorecard(
    aces: u8,
    most_cards: u8,
    most_spades: u8,
    ten_of_diamonds: u8,
    two_of_spades: u8,
    suipi_count: u8,
    total: u8,
) -> Scorecard {
    Scorecard {
        aces,
        most_cards,
        most_spades,
        suipi_count,
        ten_of_diamonds,
        two_of_spades,
        total,
    }
}

/// Get a blank scorecard
pub fn blank_scorecard() -> Scorecard {
    scorecard(0, 0, 0, 0, 0, 0, 0)
}

/// Apply a move to the game from a string annotation
pub fn apply(g: &mut Box<Game>, x: &str) -> Result<(), String> {
    let action = CString::new(String::from(x)).unwrap();
    let error = unsafe {
        CStr::from_ptr(api::apply_move(g, action.as_ptr()))
            .to_str()
            .unwrap()
    };
    if error.is_empty() {
        Ok(())
    } else {
        Err(String::from(error))
    }
}

/// Apply a set of moves to initialize game state
pub fn apply_moves(g: &mut Box<Game>, xs: Vec<&str>) {
    for x in xs {
        assert!(apply(g, x).is_ok());
        api::next_turn(g);
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

/// Helper for creating a card
pub fn card(v: Value, s: Suit) -> Card {
    Card::create(v, s)
}

/// Helper for creating an invalid card
pub fn blank() -> Card {
    Card::invalid()
}
