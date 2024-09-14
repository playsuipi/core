use playsuipi_core::card::{Suit, Value};

mod common;
use common::*;

#[test]
fn test_pair_two_cards() {
    let mut g = setup_default();

    assert!(apply(&mut g, "*C&3").is_ok());

    assert_eq!(
        read_floor(&g),
        vec![
            single(Value::Four, Suit::Clubs),
            single(Value::Seven, Suit::Diamonds),
            single(Value::Eight, Suit::Clubs),
            empty(), // single(Value::Two, Suit::Spades),
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
        read_hands(&g),
        vec![
            // Opponent hand:
            card(Value::Ace, Suit::Hearts),
            card(Value::King, Suit::Clubs),
            blank(), // card(Value::Two, Suit::Diamonds),
            card(Value::Ace, Suit::Clubs),
            card(Value::Seven, Suit::Clubs),
            card(Value::Eight, Suit::Spades),
            card(Value::King, Suit::Hearts),
            card(Value::Three, Suit::Spades),
            // Dealer hand:
            card(Value::Ten, Suit::Diamonds),
            card(Value::Four, Suit::Hearts),
            card(Value::Ten, Suit::Spades),
            card(Value::Five, Suit::Spades),
            card(Value::Three, Suit::Diamonds),
            card(Value::Five, Suit::Clubs),
            card(Value::Six, Suit::Spades),
            card(Value::Jack, Suit::Hearts),
        ]
    );

    assert_eq!(
        g.state.opponent.pairs,
        vec![pair(
            vec![
                card(Value::Two, Suit::Spades),
                card(Value::Two, Suit::Diamonds),
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
        read_floor(&g),
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
        read_hands(&g),
        vec![
            // Opponent hand:
            blank(), // card(Value::Ace, Suit::Hearts),
            card(Value::King, Suit::Clubs),
            card(Value::Two, Suit::Diamonds),
            card(Value::Ace, Suit::Clubs),
            card(Value::Seven, Suit::Clubs),
            card(Value::Eight, Suit::Spades),
            card(Value::King, Suit::Hearts),
            card(Value::Three, Suit::Spades),
            // Dealer hand:
            card(Value::Ten, Suit::Diamonds),
            card(Value::Four, Suit::Hearts),
            card(Value::Ten, Suit::Spades),
            card(Value::Five, Suit::Spades),
            card(Value::Three, Suit::Diamonds),
            card(Value::Five, Suit::Clubs),
            card(Value::Six, Suit::Spades),
            card(Value::Jack, Suit::Hearts),
        ]
    );
}

#[test]
fn test_build_and_group() {
    let mut g = setup_default();

    assert!(apply(&mut g, "D&B+1").is_ok());

    assert_eq!(
        read_floor(&g),
        vec![
            single(Value::Four, Suit::Clubs),
            single(Value::Two, Suit::Spades),
            group(
                vec![
                    card(Value::Eight, Suit::Clubs),
                    card(Value::Seven, Suit::Diamonds),
                    card(Value::Ace, Suit::Hearts),
                ],
                Value::Eight
            ), // single(Value::Eight, Suit::Clubs),
            empty(), // single(Value::Seven, Suit::Diamonds),
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
        read_hands(&g),
        vec![
            // Opponent hand:
            blank(), // card(Value::Ace, Suit::Hearts),
            card(Value::King, Suit::Clubs),
            card(Value::Two, Suit::Diamonds),
            card(Value::Ace, Suit::Clubs),
            card(Value::Seven, Suit::Clubs),
            card(Value::Eight, Suit::Spades),
            card(Value::King, Suit::Hearts),
            card(Value::Three, Suit::Spades),
            // Dealer hand:
            card(Value::Ten, Suit::Diamonds),
            card(Value::Four, Suit::Hearts),
            card(Value::Ten, Suit::Spades),
            card(Value::Five, Suit::Spades),
            card(Value::Three, Suit::Diamonds),
            card(Value::Five, Suit::Clubs),
            card(Value::Six, Suit::Spades),
            card(Value::Jack, Suit::Hearts),
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
        read_floor(&g),
        vec![
            single(Value::Five, Suit::Hearts),
            single(Value::King, Suit::Hearts),
            single(Value::Four, Suit::Spades),
            build(
                vec![
                    card(Value::Seven, Suit::Clubs),
                    card(Value::Three, Suit::Hearts),
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
        read_hands(&g),
        vec![
            // Opponent hand:
            blank(), // card(Value::Three, Suit::Hearts),
            card(Value::Four, Suit::Diamonds),
            card(Value::Queen, Suit::Hearts),
            card(Value::Eight, Suit::Diamonds),
            card(Value::King, Suit::Spades),
            card(Value::Five, Suit::Diamonds),
            card(Value::Ten, Suit::Diamonds),
            card(Value::Ten, Suit::Spades),
            // Dealer hand:
            card(Value::King, Suit::Diamonds),
            card(Value::Jack, Suit::Diamonds),
            card(Value::Four, Suit::Hearts),
            card(Value::Seven, Suit::Hearts),
            card(Value::Queen, Suit::Diamonds),
            card(Value::Six, Suit::Hearts),
            card(Value::King, Suit::Clubs),
            card(Value::Jack, Suit::Clubs),
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
        read_floor(&g),
        vec![
            single(Value::Ten, Suit::Hearts),
            single(Value::Jack, Suit::Diamonds),
            empty(), // single(Value::Four, Suit::Clubs),
            empty(), // single(Value::Five, Suit::Diamonds),
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
        read_hands(&g),
        vec![
            // Opponent hand:
            card(Value::Queen, Suit::Clubs),
            card(Value::Three, Suit::Spades),
            card(Value::Eight, Suit::Spades),
            card(Value::Ten, Suit::Diamonds),
            blank(), // card(Value::Nine, Suit::Spades),
            card(Value::Six, Suit::Clubs),
            card(Value::Ace, Suit::Spades),
            card(Value::Ten, Suit::Clubs),
            // Dealer hand:
            card(Value::Queen, Suit::Diamonds),
            card(Value::Five, Suit::Spades),
            card(Value::Seven, Suit::Diamonds),
            card(Value::Nine, Suit::Clubs),
            card(Value::Ace, Suit::Hearts),
            card(Value::Five, Suit::Hearts),
            card(Value::Six, Suit::Hearts),
            card(Value::King, Suit::Clubs),
        ]
    );

    assert_eq!(
        g.state.opponent.pairs,
        vec![pair(
            vec![
                card(Value::Four, Suit::Clubs),
                card(Value::Five, Suit::Diamonds),
                card(Value::Nine, Suit::Spades),
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
        read_floor(&g),
        vec![
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
        read_hands(&g),
        vec![
            // Opponent hand:
            card(Value::Eight, Suit::Spades),
            card(Value::Jack, Suit::Clubs),
            card(Value::Six, Suit::Clubs),
            card(Value::Eight, Suit::Hearts),
            blank(), // card(Value::Seven, Suit::Spades),
            card(Value::Five, Suit::Clubs),
            card(Value::King, Suit::Hearts),
            card(Value::Two, Suit::Hearts),
            // Dealer hand:
            card(Value::Five, Suit::Diamonds),
            card(Value::Jack, Suit::Hearts),
            card(Value::Four, Suit::Spades),
            card(Value::Three, Suit::Spades),
            card(Value::Ace, Suit::Hearts),
            card(Value::Ten, Suit::Hearts),
            card(Value::Queen, Suit::Spades),
            card(Value::Eight, Suit::Clubs),
        ]
    );

    assert_eq!(
        g.state.opponent.pairs,
        vec![pair(
            vec![
                card(Value::Three, Suit::Diamonds),
                card(Value::Four, Suit::Diamonds),
                card(Value::Five, Suit::Spades),
                card(Value::Two, Suit::Diamonds),
                card(Value::Seven, Suit::Spades),
            ],
            Value::Seven,
            Owner::Opponent,
        )]
    );
}

#[test]
fn test_first_round() {
    let mut g = setup_default();

    apply_moves(
        &mut g,
        vec![
            "*D&6", "*A+C&7", "*A&5", "!8", "!7", "!4", "*B&2", "*B&6", "!1", "B+5", "!4", "*B&2",
            "B+3", "!3", "*B&8", "*B&1",
        ],
    );

    assert_eq!(
        read_floor(&g),
        vec![
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
        read_hands(&g),
        vec![
            // Opponent hand:
            card(Value::Nine, Suit::Clubs),
            card(Value::Five, Suit::Hearts),
            card(Value::King, Suit::Spades),
            card(Value::Nine, Suit::Diamonds),
            card(Value::Ace, Suit::Diamonds),
            card(Value::Eight, Suit::Hearts),
            card(Value::Queen, Suit::Spades),
            card(Value::Nine, Suit::Spades),
            // Dealer hand:
            card(Value::Six, Suit::Hearts),
            card(Value::Jack, Suit::Clubs),
            card(Value::Four, Suit::Spades),
            card(Value::Five, Suit::Diamonds),
            card(Value::Two, Suit::Clubs),
            card(Value::Seven, Suit::Spades),
            card(Value::Queen, Suit::Diamonds),
            card(Value::Nine, Suit::Hearts),
        ]
    );

    assert_eq!(
        g.state.opponent.pairs,
        vec![
            pair(
                vec![
                    card(Value::Eight, Suit::Clubs),
                    card(Value::Eight, Suit::Spades),
                ],
                Value::Eight,
                Owner::Opponent,
            ),
            pair(
                vec![
                    card(Value::Seven, Suit::Diamonds),
                    card(Value::Seven, Suit::Clubs),
                ],
                Value::Seven,
                Owner::Opponent,
            ),
            pair(
                vec![
                    card(Value::King, Suit::Hearts),
                    card(Value::King, Suit::Clubs),
                ],
                Value::King,
                Owner::Opponent,
            ),
            pair(
                vec![
                    card(Value::Ace, Suit::Clubs),
                    card(Value::Two, Suit::Diamonds),
                    card(Value::Three, Suit::Spades),
                ],
                Value::Three,
                Owner::Opponent,
            ),
        ]
    );

    assert_eq!(
        g.state.dealer.pairs,
        vec![
            pair(
                vec![
                    card(Value::Four, Suit::Clubs),
                    card(Value::Two, Suit::Spades),
                    card(Value::Six, Suit::Spades),
                ],
                Value::Six,
                Owner::Dealer,
            ),
            pair(
                vec![
                    card(Value::Five, Suit::Spades),
                    card(Value::Five, Suit::Clubs),
                ],
                Value::Five,
                Owner::Dealer,
            ),
            pair(
                vec![
                    card(Value::Ace, Suit::Hearts),
                    card(Value::Three, Suit::Diamonds),
                    card(Value::Four, Suit::Hearts),
                ],
                Value::Four,
                Owner::Dealer,
            ),
            pair(
                vec![
                    card(Value::Ten, Suit::Spades),
                    card(Value::Ten, Suit::Diamonds),
                ],
                Value::Ten,
                Owner::Dealer,
            ),
        ]
    );
}

#[test]
fn test_another_round() {
    let mut g = setup([
        229, 206, 248, 97, 54, 114, 229, 97, 217, 93, 61, 160, 176, 231, 38, 48, 39, 92, 130, 186,
        52, 30, 115, 58, 103, 197, 243, 129, 39, 107, 203, 248,
    ]);

    apply_moves(
        &mut g,
        vec![
            "*A+D&C&8", "!5", "*A&3", "*A&3", "!5", "!2", "!1", "B+6", "C+2", "*B&8", "*B&6", "!1",
            "!7", "!4", "*C&4", "!7",
        ],
    );

    assert_eq!(
        read_floor(&g),
        vec![
            single(Value::King, Suit::Clubs), // single(Value::Four, Suit::Diamonds),
            single(Value::Queen, Suit::Diamonds), // single(Value::Nine, Suit::Diamonds),
            single(Value::Five, Suit::Hearts), // single(Value::Six, Suit::Diamonds),
            single(Value::Six, Suit::Clubs),  // single(Value::Two, Suit::Hearts),
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
        read_hands(&g),
        vec![
            // Opponent hand:
            card(Value::Ten, Suit::Clubs),
            card(Value::Four, Suit::Clubs),
            card(Value::Ace, Suit::Hearts),
            card(Value::Ace, Suit::Spades),
            card(Value::Seven, Suit::Diamonds),
            card(Value::Queen, Suit::Hearts),
            card(Value::Jack, Suit::Hearts),
            card(Value::Jack, Suit::Spades),
            // Dealer hand:
            card(Value::Eight, Suit::Diamonds),
            card(Value::Three, Suit::Spades),
            card(Value::King, Suit::Hearts),
            card(Value::Four, Suit::Hearts),
            card(Value::Six, Suit::Hearts),
            card(Value::King, Suit::Diamonds),
            card(Value::Seven, Suit::Clubs),
            card(Value::Jack, Suit::Clubs),
        ]
    );

    assert_eq!(
        g.state.opponent.pairs,
        vec![
            pair(
                vec![
                    card(Value::Four, Suit::Diamonds),
                    card(Value::Two, Suit::Hearts),
                    card(Value::Six, Suit::Diamonds),
                    card(Value::Six, Suit::Spades),
                ],
                Value::Six,
                Owner::Opponent,
            ),
            pair(
                vec![
                    card(Value::Nine, Suit::Diamonds),
                    card(Value::Nine, Suit::Spades),
                ],
                Value::Nine,
                Owner::Opponent,
            ),
            pair(
                vec![
                    card(Value::Eight, Suit::Clubs),
                    card(Value::Two, Suit::Diamonds),
                    card(Value::Ten, Suit::Spades),
                ],
                Value::Ten,
                Owner::Opponent,
            ),
            pair(
                vec![
                    card(Value::Eight, Suit::Hearts),
                    card(Value::Eight, Suit::Spades),
                ],
                Value::Eight,
                Owner::Opponent,
            ),
        ]
    );

    assert_eq!(
        g.state.dealer.pairs,
        vec![
            pair(
                vec![
                    card(Value::Seven, Suit::Hearts),
                    card(Value::Seven, Suit::Spades),
                ],
                Value::Seven,
                Owner::Dealer,
            ),
            pair(
                vec![
                    card(Value::Three, Suit::Clubs),
                    card(Value::Ace, Suit::Diamonds),
                    card(Value::Four, Suit::Spades),
                ],
                Value::Four,
                Owner::Dealer,
            ),
        ]
    );
}
