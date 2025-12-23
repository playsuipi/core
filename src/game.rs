use crate::action::Move;
use crate::rng::{Rng, Seed};
use crate::score::Score;
use crate::state::{State, StateError};

#[derive(Default)]
pub struct Game {
    pub game: u8,
    pub round: u8,
    pub rng: Rng,
    pub state: State,
    pub scores: Vec<Score>,
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
            self.scores.push(Score::from(&self.state));
            self.state.init_deck();
            self.state.shuffle_deck(self.rng.rng_borrow_mut());
            self.state.deal_hands();
            self.state.deal_floor();
        } else {
            self.state.deal_hands();
        }
    }

    /// Move the game state forward one turn
    pub fn tick(&mut self) {
        // Handle Suipi condition
        if self.state.floor_count() == 0 {
            self.state.player_mut().suipi_count += 1;
        }
        // Toggle turn
        self.state.turn = self.state.dealer.card_count() > self.state.opponent.card_count();
        // Handle end of round
        if self.state.dealer.card_count() == 0 && self.state.opponent.card_count() == 0 {
            // Handle end of game
            if self.state.deck.is_empty() {
                self.state.pickup_floor();
                self.scores[self.game as usize] = Score::from(&self.state);
                self.state = State::default();
                self.history = Vec::new();
                self.round = 0;
                self.game += 1;
            } else {
                self.round += 1;
            }
            self.deal();
        } else {
            // Bump live scoring every turn
            self.scores[self.game as usize] = Score::from(&self.state);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Annotation;
    use crate::card::{Card, Suit, Value};
    use crate::pile::{Mark, Pile};

    #[test]
    fn test_sanity() {
        // Setup with the default seed
        let mut g = Game::default();
        g.seed(Seed::default());
        g.deal();

        // Apply the move *C&3
        let m = Annotation::new(String::from("*C&3")).to_move();
        assert!(m.is_ok());
        assert!(g.apply(m.unwrap()).is_ok());

        // Check that state matches expectations
        assert_eq!(
            g.state.floor,
            vec![
                Pile::single(Card::create(Value::Four, Suit::Clubs)),
                Pile::single(Card::create(Value::Seven, Suit::Diamonds)),
                Pile::single(Card::create(Value::Eight, Suit::Clubs)),
                Pile::default(), // Pile::single(Card::create(Value::Two, Suit::Hearts)),
                Pile::default(),
                Pile::default(),
                Pile::default(),
                Pile::default(),
                Pile::default(),
                Pile::default(),
                Pile::default(),
                Pile::default(),
                Pile::default()
            ]
        );

        assert_eq!(
            g.state.opponent.hand,
            [
                Pile::single(Card::create(Value::Ace, Suit::Hearts)),
                Pile::single(Card::create(Value::King, Suit::Clubs)),
                Pile::default(), // Pile::single(Card::create(Value::Two, Suit::Diamonds)),
                Pile::single(Card::create(Value::Ace, Suit::Clubs)),
                Pile::single(Card::create(Value::Seven, Suit::Clubs)),
                Pile::single(Card::create(Value::Eight, Suit::Spades)),
                Pile::single(Card::create(Value::King, Suit::Hearts)),
                Pile::single(Card::create(Value::Three, Suit::Spades)),
            ]
        );

        assert_eq!(
            g.state.opponent.pairs,
            vec![Pile::new(
                vec![
                    Card::create(Value::Two, Suit::Hearts),
                    Card::create(Value::Two, Suit::Diamonds),
                ],
                Value::Two as u8,
                Mark::Pair,
            )]
        );

        assert_eq!(
            g.state.dealer.hand,
            [
                Pile::single(Card::create(Value::Ten, Suit::Diamonds)),
                Pile::single(Card::create(Value::Four, Suit::Hearts)),
                Pile::single(Card::create(Value::Ten, Suit::Spades)),
                Pile::single(Card::create(Value::Five, Suit::Spades)),
                Pile::single(Card::create(Value::Three, Suit::Diamonds)),
                Pile::single(Card::create(Value::Five, Suit::Clubs)),
                Pile::single(Card::create(Value::Six, Suit::Spades)),
                Pile::single(Card::create(Value::Jack, Suit::Hearts)),
            ]
        );

        assert_eq!(g.state.dealer.pairs, vec![]);
    }
}
