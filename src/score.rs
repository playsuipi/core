use crate::card::{Card, Suit, Value};
use crate::state::{Player, State};
use std::cmp::Ordering;

/// Point value winners
#[derive(Default, Eq, PartialEq)]
pub enum Winner {
    Dealer(u8),
    Opponent(u8),
    #[default]
    Tie,
}

impl Winner {
    /// Get a winner between two number values
    fn new(dealer: usize, opponent: usize, score: u8) -> Self {
        match dealer.cmp(&opponent) {
            Ordering::Equal => Winner::Tie,
            Ordering::Greater => Winner::Dealer(score),
            Ordering::Less => Winner::Opponent(score),
        }
    }

    /// Get the first winner or tie
    fn either(dealer: bool, opponent: bool, score: u8) -> Self {
        if dealer {
            Winner::Dealer(score)
        } else if opponent {
            Winner::Opponent(score)
        } else {
            Winner::Tie
        }
    }
}

/// Individual player score data
#[derive(Default)]
pub struct PlayerScore {
    pub aces: usize,
    pub suipi_count: usize,
    pub total_cards: usize,
    pub total_spades: usize,
    pub ten_of_diamonds: bool,
    pub two_of_spades: bool,
}

impl From<&Player> for PlayerScore {
    fn from(player: &Player) -> Self {
        let mut score = PlayerScore::default();
        let cards = player.into_pair_cards();
        score.aces = cards
            .iter()
            .filter(|&c| c.value == Value::Ace as u8)
            .count();
        score.suipi_count = player.suipi_count as usize;
        score.total_cards = cards.len();
        score.total_spades = cards
            .iter()
            .filter(|&c| c.suit == Suit::Spades as u8)
            .count();
        score.ten_of_diamonds = cards.contains(&Card::create(Value::Ten, Suit::Diamonds));
        score.two_of_spades = cards.contains(&Card::create(Value::Two, Suit::Spades));
        score
    }
}

/// End of game score data
#[derive(Default)]
pub struct Score {
    pub dealer_aces: u8,
    pub opponent_aces: u8,
    pub most_cards: Winner,
    pub most_spades: Winner,
    pub suipi_bonus: Winner,
    pub ten_of_diamonds: Winner,
    pub two_of_spades: Winner,
}

impl Score {
    /// Get awarded point winners
    pub fn awards(&self) -> [&Winner; 5] {
        [
            &self.most_cards,
            &self.most_spades,
            &self.suipi_bonus,
            &self.ten_of_diamonds,
            &self.two_of_spades,
        ]
    }

    /// Get an array of dealer awarded points
    pub fn dealer_points(&self) -> [u8; 5] {
        let mut scores = [0; 5];
        for (i, w) in self.awards().iter().enumerate() {
            if let Winner::Dealer(x) = w {
                scores[i] = x.to_owned();
            }
        }
        scores
    }

    /// Get an array of opponent awarded points
    pub fn opponent_points(&self) -> [u8; 5] {
        let mut scores = [0; 5];
        for (i, w) in self.awards().iter().enumerate() {
            if let Winner::Opponent(x) = w {
                scores[i] = x.to_owned();
            }
        }
        scores
    }

    /// Get the total score for the dealer
    pub fn dealer_total(&self) -> u8 {
        self.dealer_points().iter().sum::<u8>() + self.dealer_aces
    }

    /// Get the total score for the opponent
    pub fn opponent_total(&self) -> u8 {
        self.opponent_points().iter().sum::<u8>() + self.opponent_aces
    }
}

impl From<&State> for Score {
    fn from(state: &State) -> Self {
        let opp = PlayerScore::from(&state.opponent);
        let dealer = PlayerScore::from(&state.dealer);
        Score {
            dealer_aces: dealer.aces as u8,
            opponent_aces: opp.aces as u8,
            most_cards: Winner::new(dealer.total_cards, opp.total_cards, 1),
            most_spades: Winner::new(dealer.total_spades, opp.total_spades, 1),
            suipi_bonus: Winner::new(
                dealer.suipi_count,
                opp.suipi_count,
                (dealer.suipi_count as i8 - opp.suipi_count as i8).unsigned_abs(),
            ),
            ten_of_diamonds: Winner::either(dealer.ten_of_diamonds, opp.ten_of_diamonds, 2),
            two_of_spades: Winner::either(dealer.two_of_spades, opp.two_of_spades, 1),
        }
    }
}
