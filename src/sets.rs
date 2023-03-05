use crate::card::{Card, Value};
use crate::error::SuipiError;

/// Set of cards with a specific relationship
pub trait Set {
    /// Get all the cards in the set
    fn to_cards(&self) -> Vec<Card>;

    /// Get the calculated value of the set
    fn value(&self) -> Result<Value, SuipiError>;
}

// =====================
// == Single Card Set ==
// =====================

/// Wrapper for a single card
#[derive(Debug)]
pub struct Single {
    card: Card,
}

impl Single {
    /// Single out a specific card
    pub fn new(c: Card) -> Single {
        Single { card: c }
    }
}

impl Set for Single {
    fn to_cards(&self) -> Vec<Card> {
        vec![self.card]
    }

    fn value(&self) -> Result<Value, SuipiError> {
        Ok(self.card.value)
    }
}

// =====================
// == Build Cards Set ==
// =====================

/// A set of cards that add up to a sum
#[derive(Debug)]
pub struct Build {
    cards: Vec<Card>,
}

impl Build {
    /// Get a build from a set of cards
    pub fn new(xs: Vec<Card>) -> Build {
        Build { cards: xs }
    }
}

impl Set for Build {
    fn to_cards(&self) -> Vec<Card> {
        self.cards.to_owned()
    }

    fn value(&self) -> Result<Value, SuipiError> {
        match self.cards.iter().map(|x| x.value.id() + 1).sum::<u8>() {
            11.. => Err(SuipiError::InvalidBuildError),
            0 => Err(SuipiError::InvalidBuildError),
            x => Ok(Value::from_id(x - 1)?),
        }
    }
}

// =====================
// == Group Cards Set ==
// =====================

/// A group of sets with the same values
#[derive(Debug)]
pub struct Group {
    builds: Vec<Build>,
    singles: Vec<Single>,
}

impl Group {
    /// Get a group of sets with the same values
    pub fn new(b: Vec<Build>, s: Vec<Single>) -> Group {
        Group {
            builds: b,
            singles: s,
        }
    }
}

impl Set for Group {
    fn to_cards(&self) -> Vec<Card> {
        self.builds
            .iter()
            .flat_map(|x| x.to_cards())
            .chain(self.singles.iter().flat_map(|x| x.to_cards()))
            .collect::<Vec<Card>>()
    }

    fn value(&self) -> Result<Value, SuipiError> {
        let v = self
            .singles
            .iter()
            .map(|x| x.value())
            .chain(self.builds.iter().map(|x| x.value()))
            .reduce(|x, y| match (y, x) {
                (Ok(a), Ok(b)) => match a == b {
                    false => Err(SuipiError::InvalidGroupError),
                    true => Ok(a),
                },
                (Ok(_), Err(e)) => Err(e),
                (Err(e), _) => Err(e),
            });

        match v {
            None => Err(SuipiError::InvalidGroupError),
            Some(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    #[test]
    fn test_single_card_set() {
        let v = Value::Four;
        let s = Suit::Clubs;
        let single = Single::new(Card::new(v, s));
        let cards = single.to_cards();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].value, v);
        assert_eq!(cards[0].suit, s);
        assert_eq!(single.value(), Ok(v));
    }

    #[test]
    fn test_build_cards_set() {
        let xs = vec![
            Card::new(Value::Two, Suit::Spades),
            Card::new(Value::Six, Suit::Hearts),
        ];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Ok(Value::Eight));

        let xs = vec![
            Card::new(Value::Three, Suit::Diamonds),
            Card::new(Value::Four, Suit::Clubs),
        ];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Ok(Value::Seven));

        let xs = vec![
            Card::new(Value::Ace, Suit::Clubs),
            Card::new(Value::Six, Suit::Diamonds),
            Card::new(Value::Three, Suit::Spades),
        ];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Ok(Value::Ten));

        // A single card build is technically valid
        let xs = vec![Card::new(Value::Two, Suit::Spades)];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Ok(Value::Two));
    }

    #[test]
    fn test_invalid_build_cards_set() {
        let xs = vec![
            Card::new(Value::King, Suit::Diamonds),
            Card::new(Value::Queen, Suit::Hearts),
        ];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Err(SuipiError::InvalidBuildError));

        let xs = vec![];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Err(SuipiError::InvalidBuildError));
    }

    #[test]
    fn test_group_cards_set() {
        let b = vec![Build::new(vec![
            Card::new(Value::Two, Suit::Clubs),
            Card::new(Value::Three, Suit::Spades),
        ])];
        let s = vec![Single::new(Card::new(Value::Five, Suit::Hearts))];
        let g = Group::new(b, s);
        let expected = [
            Card::new(Value::Two, Suit::Clubs),
            Card::new(Value::Three, Suit::Spades),
            Card::new(Value::Five, Suit::Hearts),
        ];
        assert_eq!(g.to_cards(), expected);
        assert_eq!(g.value(), Ok(Value::Five));
    }
}
