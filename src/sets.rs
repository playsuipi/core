use crate::card::{Card, Value};
use crate::error::SuipiError;

/// Set of cards with a specific relationship
pub trait Set {
    /// Get all the cards in the set
    fn to_cards(&self) -> Vec<Card>;

    /// Get the calculated value of the set
    fn value(&self) -> Result<Value, SuipiError>;
}

/// Set of cards that can be used in a build
pub trait Buildable: Set {}

// =====================
// == Single Card Set ==
// =====================

/// Wrapper for a single card
#[derive(Copy, Clone, Debug)]
pub struct Single {
    card: Card,
}

impl Single {
    /// Single out a specific card
    pub fn new(c: Card) -> Single {
        Single { card: c }
    }
}

impl Buildable for Single {}

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

    /// Get a build from two buildable sets
    pub fn build(a: Box<&dyn Buildable>, b: Box<&dyn Buildable>) -> Build {
        Build::new(
            a.to_cards()
                .into_iter()
                .chain(b.to_cards().into_iter())
                .collect::<Vec<Card>>(),
        )
    }
}

impl Buildable for Build {}

impl Set for Build {
    fn to_cards(&self) -> Vec<Card> {
        self.cards.to_owned()
    }

    fn value(&self) -> Result<Value, SuipiError> {
        if self.cards.len() < 2 {
            Err(SuipiError::InvalidBuildError)
        } else {
            match self.cards.iter().map(|x| x.value.id() + 1).sum::<u8>() {
                11.. => Err(SuipiError::InvalidBuildError),
                0 => Err(SuipiError::InvalidBuildError),
                x => Ok(Value::from_id(x - 1)?),
            }
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
    root: Option<Single>,
}

impl Group {
    /// Get a group of sets with the same values
    pub fn new(b: Vec<Build>, r: Option<Single>) -> Group {
        Group { builds: b, root: r }
    }
}

impl Set for Group {
    fn to_cards(&self) -> Vec<Card> {
        let mut xs = self
            .builds
            .iter()
            .flat_map(|x| x.to_cards())
            .collect::<Vec<Card>>();

        if let Some(s) = self.root {
            xs.append(&mut s.to_cards());
        }

        return xs;
    }

    fn value(&self) -> Result<Value, SuipiError> {
        let mut xs = self
            .builds
            .iter()
            .map(|x| x.value())
            .collect::<Vec<Result<Value, SuipiError>>>();

        if let Some(s) = self.root {
            xs.push(s.value());
        }

        let v = xs.into_iter().reduce(|x, y| match (y, x) {
            (Ok(a), Ok(b)) => match a == b {
                false => Err(SuipiError::InvalidGroupError),
                true => Ok(a),
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), _) => Err(e),
        });

        return match v {
            None => Err(SuipiError::InvalidGroupError),
            Some(x) => x,
        };
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

        // Built using the build method
        let xs = vec![
            Card::new(Value::Six, Suit::Diamonds),
            Card::new(Value::Ace, Suit::Clubs),
            Card::new(Value::Three, Suit::Spades),
        ];
        let b = Build::build(
            Box::new(&Single::new(xs[0])),
            Box::new(&Build::new(vec![xs[1], xs[2]])),
        );
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Ok(Value::Ten));
    }

    #[test]
    fn test_invalid_build_cards_set() {
        // Build too high error
        let xs = vec![
            Card::new(Value::King, Suit::Diamonds),
            Card::new(Value::Queen, Suit::Hearts),
        ];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Err(SuipiError::InvalidBuildError));

        // Single card build error
        let xs = vec![Card::new(Value::Three, Suit::Diamonds)];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Err(SuipiError::InvalidBuildError));

        // Empty build error
        let xs = vec![];
        let b = Build::new(xs.clone());
        assert_eq!(b.to_cards(), xs);
        assert_eq!(b.value(), Err(SuipiError::InvalidBuildError));
    }

    #[test]
    fn test_group_cards_set() {
        // Group build and single
        let xs = [
            Card::new(Value::Two, Suit::Clubs),
            Card::new(Value::Three, Suit::Spades),
            Card::new(Value::Five, Suit::Hearts),
        ];
        let b = vec![Build::new(vec![xs[0], xs[1]])];
        let s = Some(Single::new(xs[2]));
        let g = Group::new(b, s);
        assert_eq!(g.to_cards(), xs);
        assert_eq!(g.value(), Ok(Value::Five));

        // Group two builds
        let xs = [
            Card::new(Value::Three, Suit::Clubs),
            Card::new(Value::Four, Suit::Diamonds),
            Card::new(Value::Six, Suit::Hearts),
            Card::new(Value::Ace, Suit::Spades),
        ];
        let b = vec![
            Build::new(vec![xs[0], xs[1]]),
            Build::new(vec![xs[2], xs[3]]),
        ];
        let g = Group::new(b, None);
        assert_eq!(g.to_cards(), xs);
        assert_eq!(g.value(), Ok(Value::Seven));
    }

    #[test]
    fn test_invalid_group_cards_set() {
        // Value mismatch with Build
        let xs = [
            Card::new(Value::Three, Suit::Hearts),
            Card::new(Value::Two, Suit::Diamonds),
            Card::new(Value::Six, Suit::Clubs),
        ];
        let b = vec![Build::new(vec![xs[0], xs[1]])];
        let s = Some(Single::new(xs[2]));
        let g = Group::new(b, s);
        assert_eq!(g.to_cards(), xs);
        assert_eq!(g.value(), Err(SuipiError::InvalidGroupError));

        // Build too high error bubble up
        let xs = [
            Card::new(Value::Jack, Suit::Hearts),
            Card::new(Value::Six, Suit::Clubs),
        ];
        let b = vec![Build::new(vec![xs[0], xs[1]])];
        let g = Group::new(b, None);
        assert_eq!(g.to_cards(), xs);
        assert_eq!(g.value(), Err(SuipiError::InvalidBuildError));

        // Single card build error bubble up
        let xs = [
            Card::new(Value::Five, Suit::Diamonds),
        ];
        let b = vec![Build::new(vec![xs[0]])];
        let g = Group::new(b, None);
        assert_eq!(g.to_cards(), xs);
        assert_eq!(g.value(), Err(SuipiError::InvalidBuildError));

        // Empty build error bubble up
        let b = vec![Build::new(vec![])];
        let g = Group::new(b, None);
        assert_eq!(g.to_cards(), vec![]);
        assert_eq!(g.value(), Err(SuipiError::InvalidBuildError));
    }
}
