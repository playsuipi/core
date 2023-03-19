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
        self.builds
            .iter()
            .flat_map(|x| x.to_cards())
            .chain(match self.root {
                Some(s) => s.to_cards(),
                None => vec![],
            })
            .collect::<Vec<Card>>()
    }

    fn value(&self) -> Result<Value, SuipiError> {
        match self
            .builds
            .iter()
            .map(|x| x.value())
            .chain(match self.root {
                Some(s) => vec![s.value()],
                None => vec![],
            })
            .collect::<Result<Vec<Value>, SuipiError>>()
        {
            Ok(xs) => {
                if xs.windows(2).all(|w| w[0] == w[1]) {
                    match xs.get(0) {
                        None => Err(SuipiError::InvalidGroupError),
                        Some(x) => Ok(*x),
                    }
                } else {
                    Err(SuipiError::InvalidGroupError)
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    /// Check that a set matches the expected values
    fn validate_set(s: Box<dyn Set>, cards: Vec<Card>, value: Result<Value, SuipiError>) {
        assert_eq!(s.to_cards(), cards);
        assert_eq!(s.value(), value);
    }

    /// Single validation helper
    fn validate_single(x: Card, v: Result<Value, SuipiError>) {
        validate_set(Box::new(Single::new(x)), vec![x], v);
    }

    /// Build validation helper
    fn validate_build(xs: Vec<Card>, v: Result<Value, SuipiError>) {
        validate_set(Box::new(Build::new(xs.clone())), xs, v);
    }

    /// Group validation helper
    fn validate_group(builds: Vec<Vec<Card>>, single: Option<Card>, v: Result<Value, SuipiError>) {
        let mut xs = vec![];
        let mut bs = vec![];
        let mut s = None;

        for mut b in builds {
            bs.push(Build::new(b.clone()));
            xs.append(&mut b);
        }

        if let Some(x) = single {
            s = Some(Single::new(x));
            xs.push(x);
        }

        validate_set(Box::new(Group::new(bs, s)), xs, v);
    }

    #[test]
    fn test_single_card_set() {
        validate_single(Card::new(Value::Four, Suit::Clubs), Ok(Value::Four));
        validate_single(Card::new(Value::Ace, Suit::Spades), Ok(Value::Ace));
        validate_single(Card::new(Value::King, Suit::Diamonds), Ok(Value::King));
    }

    #[test]
    fn test_build_set_from_two_cards() {
        validate_build(
            vec![
                Card::new(Value::Two, Suit::Spades),
                Card::new(Value::Six, Suit::Hearts),
            ],
            Ok(Value::Eight),
        );

        validate_build(
            vec![
                Card::new(Value::Three, Suit::Diamonds),
                Card::new(Value::Four, Suit::Clubs),
            ],
            Ok(Value::Seven),
        );
    }

    #[test]
    fn test_build_set_from_many_cards() {
        validate_build(
            vec![
                Card::new(Value::Ace, Suit::Clubs),
                Card::new(Value::Five, Suit::Diamonds),
                Card::new(Value::Three, Suit::Spades),
            ],
            Ok(Value::Nine),
        );

        validate_build(
            vec![
                Card::new(Value::Ace, Suit::Clubs),
                Card::new(Value::Two, Suit::Diamonds),
                Card::new(Value::Three, Suit::Spades),
                Card::new(Value::Four, Suit::Hearts),
            ],
            Ok(Value::Ten),
        );

        validate_build(
            vec![
                Card::new(Value::Ace, Suit::Clubs),
                Card::new(Value::Ace, Suit::Diamonds),
                Card::new(Value::Ace, Suit::Hearts),
                Card::new(Value::Ace, Suit::Spades),
                Card::new(Value::Two, Suit::Clubs),
                Card::new(Value::Two, Suit::Diamonds),
                Card::new(Value::Two, Suit::Hearts),
            ],
            Ok(Value::Ten),
        );
    }

    #[test]
    fn test_build_above_ten_error() {
        validate_build(
            vec![
                Card::new(Value::King, Suit::Diamonds),
                Card::new(Value::Queen, Suit::Hearts),
            ],
            Err(SuipiError::InvalidBuildError),
        );

        validate_build(
            vec![
                Card::new(Value::Six, Suit::Spades),
                Card::new(Value::Five, Suit::Clubs),
            ],
            Err(SuipiError::InvalidBuildError),
        );

        validate_build(
            vec![
                Card::new(Value::Two, Suit::Spades),
                Card::new(Value::Three, Suit::Clubs),
                Card::new(Value::Seven, Suit::Hearts),
            ],
            Err(SuipiError::InvalidBuildError),
        );
    }

    #[test]
    fn test_build_less_than_two_cards_error() {
        validate_build(
            vec![Card::new(Value::Eight, Suit::Diamonds)],
            Err(SuipiError::InvalidBuildError),
        );

        validate_build(vec![], Err(SuipiError::InvalidBuildError));
    }

    #[test]
    fn test_build_method() {
        let xs = vec![
            Card::new(Value::Six, Suit::Diamonds),
            Card::new(Value::Ace, Suit::Clubs),
            Card::new(Value::Three, Suit::Spades),
        ];
        let b = Build::build(
            Box::new(&Single::new(xs[0])),
            Box::new(&Build::new(vec![xs[1], xs[2]])),
        );
        validate_set(Box::new(b), xs, Ok(Value::Ten));
    }

    #[test]
    fn test_group_set_from_builds() {
        validate_group(
            vec![
                vec![
                    Card::new(Value::Three, Suit::Clubs),
                    Card::new(Value::Four, Suit::Diamonds),
                ],
                vec![
                    Card::new(Value::Six, Suit::Hearts),
                    Card::new(Value::Ace, Suit::Spades),
                ],
            ],
            None,
            Ok(Value::Seven),
        );

        validate_group(
            vec![
                vec![
                    Card::new(Value::Three, Suit::Clubs),
                    Card::new(Value::Four, Suit::Diamonds),
                    Card::new(Value::Three, Suit::Hearts),
                ],
                vec![
                    Card::new(Value::Five, Suit::Spades),
                    Card::new(Value::Five, Suit::Clubs),
                ],
            ],
            None,
            Ok(Value::Ten),
        );
    }

    #[test]
    fn test_group_set_with_root_card() {
        validate_group(
            vec![vec![
                Card::new(Value::Two, Suit::Clubs),
                Card::new(Value::Three, Suit::Spades),
            ]],
            Some(Card::new(Value::Five, Suit::Hearts)),
            Ok(Value::Five),
        );

        validate_group(
            vec![vec![
                Card::new(Value::Four, Suit::Hearts),
                Card::new(Value::Six, Suit::Clubs),
            ]],
            Some(Card::new(Value::Ten, Suit::Diamonds)),
            Ok(Value::Ten),
        );
    }

    #[test]
    fn test_group_set_values_mismatch_error() {
        validate_group(
            vec![
                vec![
                    Card::new(Value::Three, Suit::Clubs),
                    Card::new(Value::Four, Suit::Diamonds),
                ],
                vec![
                    Card::new(Value::Five, Suit::Spades),
                    Card::new(Value::Five, Suit::Clubs),
                ],
            ],
            None,
            Err(SuipiError::InvalidGroupError),
        );

        validate_group(
            vec![vec![
                Card::new(Value::Two, Suit::Diamonds),
                Card::new(Value::Three, Suit::Spades),
            ]],
            Some(Card::new(Value::Seven, Suit::Clubs)),
            Err(SuipiError::InvalidGroupError),
        );
    }

    #[test]
    fn test_group_set_with_single_value() {
        //
        // Although this is undefined behavior. It is technically valid for now.
        //
        validate_group(
            vec![vec![
                Card::new(Value::Four, Suit::Hearts),
                Card::new(Value::Four, Suit::Clubs),
            ]],
            None,
            Ok(Value::Eight),
        );

        validate_group(
            vec![],
            Some(Card::new(Value::Nine, Suit::Spades)),
            Ok(Value::Nine),
        );
    }

    #[test]
    fn test_group_set_no_values_error() {
        validate_group(vec![], None, Err(SuipiError::InvalidGroupError));
    }

    #[test]
    fn test_group_set_build_errors() {
        // Build above ten error
        validate_group(
            vec![vec![
                Card::new(Value::Six, Suit::Hearts),
                Card::new(Value::Five, Suit::Spades),
            ]],
            Some(Card::new(Value::Jack, Suit::Clubs)),
            Err(SuipiError::InvalidBuildError),
        );

        // Build less than two cards error
        validate_group(
            vec![vec![Card::new(Value::Queen, Suit::Hearts)]],
            Some(Card::new(Value::Queen, Suit::Diamonds)),
            Err(SuipiError::InvalidBuildError),
        );
    }
}
