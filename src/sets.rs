use crate::card::{Card, CardError, Value};
use std::fmt;

/// Set value errors
#[derive(Debug, PartialEq)]
pub enum SetError {
    TooFewCards,
    ValueTooHigh,
    ValueTooLow,
    ValueMismatch,
    InvalidCard(CardError),
}

/// Set of cards with a specific relationship
pub trait Set {
    /// Get all the cards in the set
    fn to_cards(&self) -> Vec<Card>;

    /// Get the calculated value of the set
    fn to_value(&self) -> Result<Value, SetError>;
}

impl fmt::Debug for dyn Set + 'static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} = ({})",
            self.to_value(),
            self.to_cards()
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("+")
        )
    }
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

    fn to_value(&self) -> Result<Value, SetError> {
        Ok(self.card.value)
    }
}

// =====================
// == Build Cards Set ==
// =====================

/// A set of cards that add up to a sum
#[derive(Clone, Debug)]
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

    fn to_value(&self) -> Result<Value, SetError> {
        if self.cards.len() < 2 {
            Err(SetError::TooFewCards)
        } else {
            match self.cards.iter().map(|x| x.value.id() + 1).sum::<u8>() {
                11.. => Err(SetError::ValueTooHigh),
                0 => Err(SetError::ValueTooLow),
                x => match Value::from_id(x - 1) {
                    Err(e) => Err(SetError::InvalidCard(e)),
                    Ok(y) => Ok(y),
                },
            }
        }
    }
}

// =====================
// == Group Cards Set ==
// =====================

/// A group of sets with the same values
#[derive(Clone, Debug)]
pub struct Group {
    builds: Vec<Build>,
    root: Option<Single>,
}

impl Group {
    /// Get a group of sets with the same values
    pub fn new(b: Vec<Build>, r: Option<Single>) -> Group {
        Group { builds: b, root: r }
    }

    /// Get a group from two different groups
    pub fn group(a: Group, b: Group) -> Group {
        Group::new(
            [a.builds.clone(), b.builds.clone()].concat(),
            match (a.root, b.root) {
                (None, None) => None,
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (Some(_), Some(_)) => None, // Undefined behavior
            },
        )
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

    fn to_value(&self) -> Result<Value, SetError> {
        match self
            .builds
            .iter()
            .map(|x| x.to_value())
            .chain(match self.root {
                Some(s) => vec![s.to_value()],
                None => vec![],
            })
            .collect::<Result<Vec<Value>, SetError>>()
        {
            Ok(xs) => {
                if xs.windows(2).all(|w| w[0] == w[1]) {
                    match xs.get(0) {
                        None => Err(SetError::TooFewCards),
                        Some(x) => Ok(*x),
                    }
                } else {
                    Err(SetError::ValueMismatch)
                }
            }
            Err(e) => Err(e),
        }
    }
}

// ====================
// == Pair Cards Set ==
// ====================

/// A set of cards paired with a single capturing card
#[derive(Debug)]
pub struct Pair {
    target: Box<dyn Set>,
    capture: Single,
}

impl Pair {
    /// Get a pair from a group and a single
    pub fn new(t: Box<dyn Set>, c: Single) -> Pair {
        Pair {
            target: t,
            capture: c,
        }
    }
}

impl Set for Pair {
    fn to_cards(&self) -> Vec<Card> {
        [self.target.to_cards(), self.capture.to_cards()].concat()
    }

    fn to_value(&self) -> Result<Value, SetError> {
        if self.target.to_value()? == self.capture.to_value()? {
            self.capture.to_value()
        } else {
            Err(SetError::ValueMismatch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    /// Check that a set matches the expected values
    fn validate_set(s: Box<dyn Set>, cards: Vec<Card>, value: Result<Value, SetError>) {
        assert_eq!(s.to_cards(), cards);
        assert_eq!(s.to_value(), value);
    }

    /// Single validation helper
    fn validate_single(x: Card, v: Result<Value, SetError>) {
        validate_set(Box::new(Single::new(x)), vec![x], v);
    }

    /// Build validation helper
    fn validate_build(xs: Vec<Card>, v: Result<Value, SetError>) {
        validate_set(Box::new(Build::new(xs.clone())), xs, v);
    }

    /// Group validation helper
    fn validate_group(builds: Vec<Vec<Card>>, single: Option<Card>, v: Result<Value, SetError>) {
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

    /// Pair validation helper
    fn validate_pair(target: Box<dyn Set>, capture: Card, v: Result<Value, SetError>) {
        let mut xs = target.to_cards();
        xs.push(capture);

        validate_set(Box::new(Pair::new(target, Single::new(capture))), xs, v);
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
    fn test_build_value_too_high_error() {
        validate_build(
            vec![
                Card::new(Value::King, Suit::Diamonds),
                Card::new(Value::Queen, Suit::Hearts),
            ],
            Err(SetError::ValueTooHigh),
        );

        validate_build(
            vec![
                Card::new(Value::Six, Suit::Spades),
                Card::new(Value::Five, Suit::Clubs),
            ],
            Err(SetError::ValueTooHigh),
        );

        validate_build(
            vec![
                Card::new(Value::Two, Suit::Spades),
                Card::new(Value::Three, Suit::Clubs),
                Card::new(Value::Seven, Suit::Hearts),
            ],
            Err(SetError::ValueTooHigh),
        );
    }

    #[test]
    fn test_build_too_few_cards_error() {
        validate_build(
            vec![Card::new(Value::Eight, Suit::Diamonds)],
            Err(SetError::TooFewCards),
        );

        validate_build(vec![], Err(SetError::TooFewCards));
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
    fn test_group_value_mismatch_error() {
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
            Err(SetError::ValueMismatch),
        );

        validate_group(
            vec![vec![
                Card::new(Value::Two, Suit::Diamonds),
                Card::new(Value::Three, Suit::Spades),
            ]],
            Some(Card::new(Value::Seven, Suit::Clubs)),
            Err(SetError::ValueMismatch),
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
    fn test_group_too_few_cards_error() {
        validate_group(vec![], None, Err(SetError::TooFewCards));
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
            Err(SetError::ValueTooHigh),
        );

        // Build less than two cards error
        validate_group(
            vec![vec![Card::new(Value::Queen, Suit::Hearts)]],
            Some(Card::new(Value::Queen, Suit::Diamonds)),
            Err(SetError::TooFewCards),
        );
    }

    #[test]
    fn test_group_method() {
        let xs = vec![
            Card::new(Value::Two, Suit::Spades),
            Card::new(Value::Four, Suit::Spades),
            Card::new(Value::Three, Suit::Clubs),
            Card::new(Value::Three, Suit::Diamonds),
            Card::new(Value::Five, Suit::Hearts),
            Card::new(Value::Ace, Suit::Spades),
            Card::new(Value::Six, Suit::Hearts),
        ];

        let a = Group::new(
            vec![Build::new(vec![xs[0], xs[1]])],
            Some(Single::new(xs[6])),
        );

        let b = Group::new(
            vec![
                Build::new(vec![xs[2], xs[3]]),
                Build::new(vec![xs[4], xs[5]]),
            ],
            None,
        );

        validate_set(Box::new(Group::group(a, b)), xs, Ok(Value::Six));
    }

    #[test]
    fn test_pair_set_from_single() {
        validate_pair(
            Box::new(Single::new(Card::new(Value::Five, Suit::Hearts))),
            Card::new(Value::Five, Suit::Diamonds),
            Ok(Value::Five),
        );

        validate_pair(
            Box::new(Single::new(Card::new(Value::Queen, Suit::Clubs))),
            Card::new(Value::Queen, Suit::Spades),
            Ok(Value::Queen),
        );

        validate_pair(
            Box::new(Single::new(Card::new(Value::Ace, Suit::Diamonds))),
            Card::new(Value::Ace, Suit::Clubs),
            Ok(Value::Ace),
        );
    }

    #[test]
    fn test_pair_set_from_build() {
        validate_pair(
            Box::new(Build::new(vec![
                Card::new(Value::Four, Suit::Clubs),
                Card::new(Value::Six, Suit::Hearts),
            ])),
            Card::new(Value::Ten, Suit::Diamonds),
            Ok(Value::Ten),
        );

        validate_pair(
            Box::new(Build::new(vec![
                Card::new(Value::Ace, Suit::Spades),
                Card::new(Value::Four, Suit::Diamonds),
                Card::new(Value::Two, Suit::Spades),
            ])),
            Card::new(Value::Seven, Suit::Hearts),
            Ok(Value::Seven),
        );

        validate_pair(
            Box::new(Build::new(vec![
                Card::new(Value::Ace, Suit::Clubs),
                Card::new(Value::Ace, Suit::Hearts),
            ])),
            Card::new(Value::Two, Suit::Hearts),
            Ok(Value::Two),
        );
    }

    #[test]
    fn test_pair_set_from_group() {
        validate_pair(
            Box::new(Group::new(
                vec![],
                Some(Single::new(Card::new(Value::Ace, Suit::Spades))),
            )),
            Card::new(Value::Ace, Suit::Hearts),
            Ok(Value::Ace),
        );

        validate_pair(
            Box::new(Group::new(
                vec![Build::new(vec![
                    Card::new(Value::Three, Suit::Hearts),
                    Card::new(Value::Five, Suit::Diamonds),
                ])],
                None,
            )),
            Card::new(Value::Eight, Suit::Clubs),
            Ok(Value::Eight),
        );

        validate_pair(
            Box::new(Group::new(
                vec![Build::new(vec![
                    Card::new(Value::Two, Suit::Spades),
                    Card::new(Value::Two, Suit::Clubs),
                    Card::new(Value::Five, Suit::Clubs),
                    Card::new(Value::Ace, Suit::Hearts),
                ])],
                Some(Single::new(Card::new(Value::Ten, Suit::Spades))),
            )),
            Card::new(Value::Ten, Suit::Diamonds),
            Ok(Value::Ten),
        );

        validate_pair(
            Box::new(Group::new(
                vec![
                    Build::new(vec![
                        Card::new(Value::Two, Suit::Hearts),
                        Card::new(Value::Four, Suit::Diamonds),
                    ]),
                    Build::new(vec![
                        Card::new(Value::Three, Suit::Spades),
                        Card::new(Value::Three, Suit::Clubs),
                    ]),
                ],
                Some(Single::new(Card::new(Value::Six, Suit::Hearts))),
            )),
            Card::new(Value::Six, Suit::Spades),
            Ok(Value::Six),
        );
    }

    #[test]
    fn test_pair_value_mismatch_error() {
        validate_pair(
            Box::new(Single::new(Card::new(Value::Six, Suit::Hearts))),
            Card::new(Value::King, Suit::Diamonds),
            Err(SetError::ValueMismatch),
        );

        validate_pair(
            Box::new(Build::new(vec![
                Card::new(Value::Two, Suit::Spades),
                Card::new(Value::Three, Suit::Hearts),
            ])),
            Card::new(Value::Four, Suit::Diamonds),
            Err(SetError::ValueMismatch),
        );

        validate_pair(
            Box::new(Group::new(
                vec![Build::new(vec![
                    Card::new(Value::Two, Suit::Diamonds),
                    Card::new(Value::Five, Suit::Clubs),
                ])],
                Some(Single::new(Card::new(Value::Seven, Suit::Hearts))),
            )),
            Card::new(Value::Six, Suit::Clubs),
            Err(SetError::ValueMismatch),
        );
    }

    #[test]
    fn test_pair_set_child_errors() {
        validate_pair(
            Box::new(Build::new(vec![
                Card::new(Value::Ten, Suit::Clubs),
                Card::new(Value::Two, Suit::Diamonds),
            ])),
            Card::new(Value::Queen, Suit::Hearts),
            Err(SetError::ValueTooHigh),
        );

        validate_pair(
            Box::new(Group::new(vec![], None)),
            Card::new(Value::Ace, Suit::Hearts),
            Err(SetError::TooFewCards),
        );

        validate_pair(
            Box::new(Group::new(
                vec![Build::new(vec![
                    Card::new(Value::Four, Suit::Spades),
                    Card::new(Value::Four, Suit::Hearts),
                ])],
                Some(Single::new(Card::new(Value::Seven, Suit::Clubs))),
            )),
            Card::new(Value::Eight, Suit::Hearts),
            Err(SetError::ValueMismatch),
        );
    }
}
