use crate::card::{Card, IdentyError, Value};

/// Set of cards with a specific relationship
pub trait Set {
    /// Get all the cards in the set
    fn to_cards(&self) -> Vec<Card>;

    /// Get the calculated value of the set
    fn value(&self) -> Result<Value, IdentyError>;
}

// =====================
// == Single Card Set ==
// =====================

/// Wrapper for a single card
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

    fn value(&self) -> Result<Value, IdentyError> {
        Ok(self.card.value)
    }
}

// =====================
// == Build Cards Set ==
// =====================

/// A set of cards that add up to a sum
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

    fn value(&self) -> Result<Value, IdentyError> {
        Ok(Value::from_id(
            self.cards.iter().map(|x| x.value.id() + 1).sum::<u8>() - 1,
        )?)
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
    }
}
