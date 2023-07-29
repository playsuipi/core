use playsuipi_core::card::{Card, Suit, Value};

mod common;
use common::*;

//
// These are the tests from my Haskell project. I figure they will make a decent starting point
// when creating a set of comprehensive rules.
//
// Build Module Tests:
//   Cards can be used to build a group: [OK]
//   Cards can be used to build a stack: [OK]
//   Stacks can be used to build a group: [OK]
//   Stacks can be used to build a stack: [OK]
//   Groups can be used to build a group: [OK]
//   Groups can NOT be used to build a stack: [OK]
//   Face cards can NOT be used to build a group: [OK]
//   Face cards can NOT be used to build a stack: [OK]
//   Buildables with different values can NOT be grouped: [OK]
//   Stacks can NOT be built to a value higher than 10: [OK]
//   Cards with the same value can NOT be stacked: [OK]
// Card Module Tests:
//   Card values are comparable: [OK]
//   Card suits are comparable: [OK]
//   Cards are comparable: [OK]
//   Card values can convert to ints: [OK]
// Pair Module Tests:
//   Pair calculates the total card count: [OK]
//   Pair calculates the total spade count: [OK]
//   Pair calculates the total ace count: [OK]
//

#[test]
fn test_pair_two_cards() {
    let mut g = setup_default();

    assert!(apply(&mut g, "*C&3").is_ok());

    assert_eq!(
        g.floor,
        [
            single(Value::Four, Suit::Clubs),
            single(Value::Seven, Suit::Diamonds),
            empty(), // single(Value::Two, Suit::Spades),
            single(Value::Eight, Suit::Clubs),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty()
        ]
    );

    assert_eq!(
        g.opponent.hand,
        [
            single(Value::Ace, Suit::Hearts),
            single(Value::King, Suit::Clubs),
            empty(), // single(Value::Two, Suit::Diamonds),
            single(Value::Ace, Suit::Clubs),
            single(Value::Seven, Suit::Clubs),
            single(Value::Eight, Suit::Spades),
            single(Value::King, Suit::Hearts),
            single(Value::Three, Suit::Spades),
        ]
    );

    assert_eq!(
        g.opponent.pairs.take(),
        vec![pair(
            vec![
                Card::create(Value::Two, Suit::Spades),
                Card::create(Value::Two, Suit::Diamonds),
            ],
            Value::Two
        )]
    );
}

#[test]
fn test_discard_from_hand() {
    let mut g = setup_default();

    assert!(apply(&mut g, "!1").is_ok());

    assert_eq!(
        g.floor,
        [
            single(Value::Four, Suit::Clubs),
            single(Value::Seven, Suit::Diamonds),
            single(Value::Two, Suit::Spades),
            single(Value::Eight, Suit::Clubs),
            single(Value::Ace, Suit::Hearts), // empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty()
        ]
    );

    assert_eq!(
        g.opponent.hand,
        [
            empty(), // single(Value::Ace, Suit::Hearts),
            single(Value::King, Suit::Clubs),
            single(Value::Two, Suit::Diamonds),
            single(Value::Ace, Suit::Clubs),
            single(Value::Seven, Suit::Clubs),
            single(Value::Eight, Suit::Spades),
            single(Value::King, Suit::Hearts),
            single(Value::Three, Suit::Spades),
        ]
    );
}

#[test]
fn test_build_and_group() {
    let mut g = setup_default();

    assert!(apply(&mut g, "D&B+1").is_ok());

    assert_eq!(
        g.floor,
        [
            single(Value::Four, Suit::Clubs),
            empty(), // single(Value::Seven, Suit::Diamonds),
            single(Value::Two, Suit::Spades),
            group(
                vec![
                    Card::create(Value::Eight, Suit::Clubs),
                    Card::create(Value::Seven, Suit::Diamonds),
                    Card::create(Value::Ace, Suit::Hearts),
                ],
                Value::Eight
            ), // single(Value::Eight, Suit::Clubs),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty()
        ]
    );

    assert_eq!(
        g.opponent.hand,
        [
            empty(), // single(Value::Ace, Suit::Hearts),
            single(Value::King, Suit::Clubs),
            single(Value::Two, Suit::Diamonds),
            single(Value::Ace, Suit::Clubs),
            single(Value::Seven, Suit::Clubs),
            single(Value::Eight, Suit::Spades),
            single(Value::King, Suit::Hearts),
            single(Value::Three, Suit::Spades),
        ]
    );
}

#[test]
fn test_build_two_cards() {
    let mut g = setup([
        62, 136, 82, 123, 15, 88, 230, 198, 158, 233, 24, 104, 252, 215, 233, 118, 133, 47, 6, 62,
        194, 3, 157, 203, 232, 173, 255, 143, 129, 252, 162, 20,
    ]);

    assert!(apply(&mut g, "D+1").is_ok());

    assert_eq!(
        g.floor,
        [
            single(Value::Five, Suit::Hearts),
            single(Value::King, Suit::Hearts),
            single(Value::Four, Suit::Spades),
            build(
                vec![
                    Card::create(Value::Seven, Suit::Clubs),
                    Card::create(Value::Three, Suit::Hearts),
                ],
                Value::Ten
            ), // single(Value::Seven, Suit::Clubs),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty()
        ]
    );

    assert_eq!(
        g.opponent.hand,
        [
            empty(), // single(Value::Three, Suit::Hearts),
            single(Value::Four, Suit::Diamonds),
            single(Value::Queen, Suit::Hearts),
            single(Value::Eight, Suit::Diamonds),
            single(Value::King, Suit::Spades),
            single(Value::Five, Suit::Diamonds),
            single(Value::Ten, Suit::Diamonds),
            single(Value::Ten, Suit::Spades),
        ]
    );
}
