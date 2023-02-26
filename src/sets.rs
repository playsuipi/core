use crate::card::{Card, Value};

/// Set of cards with a specific relationship
pub trait Set {
    /// Get all the cards in the set
    fn to_cards(&self) -> Vec<&Card>;

    /// Get the calculated value of the set
    fn value(&self) -> Value;
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
    fn to_cards(&self) -> Vec<&Card> {
        vec![&self.card]
    }

    fn value(&self) -> Value {
        self.card.value
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
        assert_eq!(single.value(), v);
    }
}
