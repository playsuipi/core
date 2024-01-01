use crate::action::Move;
use crate::rng::{Rng, Seed};
use crate::state::{State, StateError};

#[derive(Default)]
pub struct Game {
    pub game: u8,
    pub round: u8,
    pub rng: Rng,
    pub state: State,
    history: Vec<State>,
}

impl Game {
    /// Initialize a game with a RNG seed value
    pub fn seed(&mut self, seed: Seed) {
        self.rng = Rng::from_seed(seed);
    }

    /// Deal cards for a new round
    pub fn deal(&mut self) {
        if self.round == 0 {
            self.state.init_deck();
            self.state.shuffle_deck(self.rng.borrow_mut());
            self.state.deal_floor();
        }
        self.state.deal_hands();
    }

    /// Move the game state forward one turn
    pub fn tick(&mut self) {
        self.state.collapse_floor();
        if self.state.deck.is_empty() {
            self.round = 0;
            self.game += 1;
        }
        if self.state.dealer.card_count() == 0 && self.state.opponent.card_count() == 0 {
            self.round += 1;
            self.deal();
        }
    }

    /// Attempt to replace the current game state with the previous one
    pub fn undo(&mut self) -> Option<State> {
        let next = self.state.clone();
        match self.history.pop() {
            Some(prev) => {
                self.state = prev;
                Some(next)
            }
            None => None,
        }
    }

    /// Attempt to apply a move to the current game state
    pub fn apply(&mut self, m: Move) -> Result<(), StateError> {
        self.history.push(self.state.clone());
        if let Err(e) = self.state.apply(m) {
            self.undo();
            Err(e)
        } else {
            Ok(())
        }
    }
}
