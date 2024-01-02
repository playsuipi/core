use crate::action::Annotation;
use crate::card::Card;
use crate::game::Game;
use crate::pile::Mark;
use crate::rng::Seed;
use std::ffi::CString;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pile {
    pub cards: [u8; 32],
    pub value: u8,
    pub build: bool,
    pub owner: bool,
}

impl Pile {
    pub fn new() -> Self {
        Pile {
            cards: [u8::from(Card::invalid()); 32],
            value: 0,
            build: false,
            owner: false,
        }
    }
}

#[repr(C)]
pub struct Status {
    pub game: u8,
    pub round: u8,
    pub turn: bool,
    pub seed: Seed,
}

/// Initialize a new game from the given seed
pub extern "C" fn new_game(seed: *const Seed) -> Box<Game> {
    let mut g = Game::default();
    if seed != ptr::null() {
        unsafe {
            g.seed(*seed);
        }
    }
    g.deal();
    Box::new(g)
}

/// Get the status signals for a game
pub extern "C" fn status(g: &Box<Game>) -> Box<Status> {
    Box::new(Status {
        game: g.game,
        round: g.round,
        turn: g.state.turn,
        seed: g.rng.borrow().get_seed(),
    })
}

/// Read the current player's hand
pub extern "C" fn read_hand(g: &Box<Game>) -> Box<[u8; 8]> {
    let mut cards = [0; 8];
    let p = g.state.player();
    for i in 0..8 {
        cards[i] = u8::from(
            p.hand[i]
                .borrow()
                .cards
                .first()
                .unwrap_or(&Card::invalid())
                .to_owned(),
        );
    }
    Box::new(cards)
}

/// Read the current floor piles
pub extern "C" fn read_floor(g: &Box<Game>) -> Box<[Pile; 13]> {
    let mut piles = [Pile::new(); 13];
    for i in 0..13 {
        let f = g.state.floor[i].borrow();
        piles[i].value = f.value;
        piles[i].build = f.mark == Mark::Build;
        piles[i].owner = f.owner;
        for (j, c) in f.cards.iter().enumerate() {
            piles[i].cards[j] = u8::from(c.to_owned());
        }
    }
    Box::new(piles)
}

/// Attempt to apply a move to the game state
pub extern "C" fn apply_move(g: &mut Box<Game>, a: &CString) -> Box<CString> {
    Box::new(
        CString::new(if let Ok(annotation) = a.to_str() {
            match Annotation::new(String::from(annotation)).to_move() {
                Err(e) => e.to_string(),
                Ok(m) => {
                    if let Err(e) = g.apply(m) {
                        e.to_string()
                    } else {
                        "".to_string() // Ok
                    }
                }
            }
        } else {
            "Error: Invalid CString".to_string()
        })
        .unwrap(),
    )
}

/// End the current player's turn
pub extern "C" fn next_turn(g: &mut Box<Game>) {
    g.state.turn = g.state.dealer.card_count() > g.state.opponent.card_count();
    g.tick();
}

/// Undo the most recent move
pub extern "C" fn undo(g: &mut Box<Game>) {
    g.undo();
}
