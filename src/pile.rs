use crate::card::Card;
use std::fmt;

/// Pile manipulation errors
#[derive(Debug, Eq, PartialEq)]
pub enum PileError {
    InvalidBuild,
    InvalidGroup,
}

/// A pile type marker
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mark {
    Empty,
    Single,
    Build,
    Group,
    Pair,
}

impl Default for Mark {
    fn default() -> Mark {
        Mark::Empty
    }
}

/// A pile of cards
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Pile {
    pub cards: Vec<Card>,
    pub value: u8,
    pub mark: Mark,
}

impl Pile {
    /// Create a new pile
    pub fn new(cards: Vec<Card>, value: u8, mark: Mark) -> Self {
        Pile { cards, value, mark }
    }

    /// Create an empty pile
    pub fn empty() -> Self {
        Pile::default()
    }

    /// Create a single card pile
    pub fn single(card: Card) -> Self {
        Pile::new(vec![card], card.value, Mark::Single)
    }

    /// Create a card in a single card pile
    pub fn card(value: u8, suit: u8) -> Self {
        Pile::single(Card::new(value, suit))
    }

    /// Is this pile empty?
    pub fn is_empty(&self) -> bool {
        self.mark == Mark::Empty
    }

    /// Can I use this pile in a build?
    pub fn buildable(x: &Pile) -> bool {
        x.mark == Mark::Single || x.mark == Mark::Build
    }

    /// Can I use this pile in a group?
    pub fn groupable(x: &Pile) -> bool {
        x.mark == Mark::Group || x.mark == Mark::Build || x.mark == Mark::Single
    }

    /// Can I pair this pile with another one?
    pub fn pairable(x: &Pile) -> bool {
        x.mark == Mark::Single
    }

    /// Are both piles singles?
    pub fn both_singles(x: &Pile, y: &Pile) -> bool {
        (x.mark == Mark::Single) && (y.mark == Mark::Single)
    }

    /// Combine the cards from two piles
    pub fn cards(x: &mut Pile, y: &mut Pile) -> Vec<Card> {
        let mut cards = vec![];
        cards.append(&mut x.cards);
        cards.append(&mut y.cards);
        cards
    }

    /// Create a build pile from two buildable piles
    pub fn build(x: &mut Pile, y: &mut Pile) -> Result<Pile, PileError> {
        assert_ne!(x.value, y.value); // Same value must group/pair
        assert!(x.value + y.value <= 10); // Build higher than 10
        assert!(Pile::buildable(x)); // Invalid left arg
        assert!(Pile::buildable(y)); // Invalid right arg
        Ok(Pile::new(Pile::cards(x, y), x.value + y.value, Mark::Build))
    }

    /// Create a group pile from two groupable piles
    pub fn group(x: &mut Pile, y: &mut Pile) -> Result<Pile, PileError> {
        assert_eq!(x.value, y.value); // Mismatched value
        assert!(!Pile::both_singles(x, y)); // More than one single
        assert!(Pile::groupable(x)); // Invalid left arg
        assert!(Pile::groupable(y)); // Invalid right arg
        Ok(Pile::new(Pile::cards(x, y), x.value, Mark::Group))
    }

    /// Create a pair pile using a pairable pile
    pub fn pair(x: &mut Pile, y: &mut Pile) -> Result<Pile, PileError> {
        assert_eq!(x.value, y.value); // Mismatched value
        assert!(Pile::pairable(y)); // Invalid right arg
        Ok(Pile::new(Pile::cards(x, y), x.value, Mark::Pair))
    }
}

impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.mark {
            Mark::Empty => write!(f, "___"),
            Mark::Single => write!(f, "{}", self.cards.first().unwrap()),
            Mark::Build => write!(f, "B{{{}}}", self.value),
            Mark::Group => write!(f, "G[{}]", self.value),
            Mark::Pair => write!(f, "P<{}>", self.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let x = Pile::default();
        assert_eq!(x.cards.len(), 0);
        assert_eq!(x.value, 0);
        assert_eq!(x.mark, Mark::Empty);
    }

    #[test]
    fn test_stuff() {
        let mut x = Pile::card(2, 0);
        let mut y = Pile::card(3, 0);
        let z = Pile::build(&mut x, &mut y);
        assert_eq!(
            z,
            Ok(Pile::new(
                vec![Card::new(2, 0), Card::new(3, 0),],
                5,
                Mark::Build
            ))
        );
        let mut a = Pile::card(5, 0);
        let b = Pile::group(&mut z.unwrap(), &mut a);
        assert_eq!(
            b,
            Ok(Pile::new(
                vec![Card::new(2, 0), Card::new(3, 0), Card::new(5, 0),],
                5,
                Mark::Group
            ))
        );
    }
}
