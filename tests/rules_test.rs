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
            Value::Two,
            Owner::Opponent,
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

#[test]
fn test_build_and_pair() {
    let mut g = setup([
        139, 195, 37, 107, 143, 64, 106, 43, 179, 69, 244, 179, 23, 204, 20, 89, 184, 125, 65, 94,
        157, 229, 237, 65, 32, 138, 107, 48, 253, 118, 170, 37,
    ]);

    assert!(apply(&mut g, "*B+C&5").is_ok());

    assert_eq!(
        g.floor,
        [
            single(Value::Ten, Suit::Hearts),
            empty(), // single(Value::Four, Suit::Clubs),
            empty(), // single(Value::Five, Suit::Diamonds),
            single(Value::Jack, Suit::Diamonds),
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
            single(Value::Queen, Suit::Clubs),
            single(Value::Three, Suit::Spades),
            single(Value::Eight, Suit::Spades),
            single(Value::Ten, Suit::Diamonds),
            empty(), // single(Value::Nine, Suit::Spades),
            single(Value::Six, Suit::Clubs),
            single(Value::Ace, Suit::Spades),
            single(Value::Ten, Suit::Clubs),
        ]
    );

    assert_eq!(
        g.opponent.pairs.take(),
        vec![pair(
            vec![
                Card::create(Value::Four, Suit::Clubs),
                Card::create(Value::Five, Suit::Diamonds),
                Card::create(Value::Nine, Suit::Spades),
            ],
            Value::Nine,
            Owner::Opponent,
        )]
    );
}

#[test]
fn test_build_and_group_then_pair() {
    let mut g = setup([
        29, 247, 241, 44, 232, 99, 201, 142, 36, 1, 16, 27, 195, 115, 96, 251, 159, 80, 23, 166,
        203, 176, 34, 17, 0, 244, 182, 172, 34, 16, 25, 255,
    ]);

    assert!(apply(&mut g, "*A+B&C+D&5").is_ok());

    assert_eq!(
        g.floor,
        [
            empty(), // single(Value::Three, Suit::Diamonds),
            empty(), // single(Value::Four, Suit::Diamonds),
            empty(), // single(Value::Five, Suit::Spades),
            empty(), // single(Value::Two, Suit::Diamonds),
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
            single(Value::Eight, Suit::Spades),
            single(Value::Jack, Suit::Clubs),
            single(Value::Six, Suit::Clubs),
            single(Value::Eight, Suit::Hearts),
            empty(), // single(Value::Seven, Suit::Spades),
            single(Value::Five, Suit::Clubs),
            single(Value::King, Suit::Hearts),
            single(Value::Two, Suit::Hearts),
        ]
    );

    assert_eq!(
        g.opponent.pairs.take(),
        vec![pair(
            vec![
                Card::create(Value::Three, Suit::Diamonds),
                Card::create(Value::Four, Suit::Diamonds),
                Card::create(Value::Five, Suit::Spades),
                Card::create(Value::Two, Suit::Diamonds),
                Card::create(Value::Seven, Suit::Spades),
            ],
            Value::Seven,
            Owner::Opponent,
        )]
    );
}

#[test]
fn test_first_round() {
    let mut g = setup_default();

    let moves = [
        "*D&6", "*A+C&7", "*A&5", "!8", "!7", "!4", "*B&2", "*B&6", "!1", "B+5", "!4", "*B&2",
        "B+3", "!3", "*B&8", "*B&1",
    ];

    for m in moves {
        assert!(apply(&mut g, m).is_ok());
        g.collapse_floor();
        g.turn = !g.turn;
    }

    assert_eq!(
        g.floor,
        [
            single(Value::Jack, Suit::Hearts), // single(Value::Four, Suit::Clubs),
            empty(),                           // single(Value::Seven, Suit::Diamonds),
            empty(),                           // single(Value::Two, Suit::Spades),
            empty(),                           // single(Value::Eight, Suit::Clubs),
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
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
        ]
    );

    assert_eq!(
        g.dealer.hand,
        [
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
        ]
    );

    assert_eq!(
        g.opponent.pairs.take(),
        vec![
            pair(
                vec![
                    Card::create(Value::Eight, Suit::Clubs),
                    Card::create(Value::Eight, Suit::Spades),
                ],
                Value::Eight,
                Owner::Opponent,
            ),
            pair(
                vec![
                    Card::create(Value::Seven, Suit::Diamonds),
                    Card::create(Value::Seven, Suit::Clubs),
                ],
                Value::Seven,
                Owner::Opponent,
            ),
            pair(
                vec![
                    Card::create(Value::King, Suit::Hearts),
                    Card::create(Value::King, Suit::Clubs),
                ],
                Value::King,
                Owner::Opponent,
            ),
            pair(
                vec![
                    Card::create(Value::Ace, Suit::Clubs),
                    Card::create(Value::Two, Suit::Diamonds),
                    Card::create(Value::Three, Suit::Spades),
                ],
                Value::Three,
                Owner::Opponent,
            ),
        ]
    );

    assert_eq!(
        g.dealer.pairs.take(),
        vec![
            pair(
                vec![
                    Card::create(Value::Four, Suit::Clubs),
                    Card::create(Value::Two, Suit::Spades),
                    Card::create(Value::Six, Suit::Spades),
                ],
                Value::Six,
                Owner::Dealer,
            ),
            pair(
                vec![
                    Card::create(Value::Five, Suit::Spades),
                    Card::create(Value::Five, Suit::Clubs),
                ],
                Value::Five,
                Owner::Dealer,
            ),
            pair(
                vec![
                    Card::create(Value::Ace, Suit::Hearts),
                    Card::create(Value::Three, Suit::Diamonds),
                    Card::create(Value::Four, Suit::Hearts),
                ],
                Value::Four,
                Owner::Dealer,
            ),
            pair(
                vec![
                    Card::create(Value::Ten, Suit::Spades),
                    Card::create(Value::Ten, Suit::Diamonds),
                ],
                Value::Ten,
                Owner::Dealer,
            ),
        ]
    );
}
