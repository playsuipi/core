use std::fmt;

const SUITS: [&str; 4] = ["♣", "♦", "♥", "♠"];
const VALUES: [&str; 14] = [
    "?", "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

/// A playing card value
pub enum Value {
    Invalid = 0,
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

/// A playing card suit
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

/// A playing card
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

impl Card {
    /// Get a card from two ints
    pub fn new(value: u8, suit: u8) -> Self {
        Card { value, suit }
    }

    /// Get a card from a value and a suit
    pub fn create(value: Value, suit: Suit) -> Self {
        Card::new(value as u8, suit as u8)
    }
}

impl From<u8> for Card {
    fn from(id: u8) -> Self {
        Card::new((id % 13) + 1, id / 13)
    }
}

impl From<Card> for u8 {
    fn from(c: Card) -> Self {
        (c.suit * 13) + c.value - 1
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            VALUES[self.value as usize], SUITS[self.suit as usize]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from() {
        // Ace of Spades is id 39
        let id: u8 = 39; // 13 * 3 + 0
        assert_eq!(Card::from(id), Card::create(Value::Ace, Suit::Spades));

        // Two of Spades is id 40
        let id: u8 = 40; // 13 * 3 + 1
        assert_eq!(Card::from(id), Card::create(Value::Two, Suit::Spades));

        // Ten of Diamonds is id 22
        let id: u8 = 22; // 13 * 1 + 9
        assert_eq!(Card::from(id), Card::create(Value::Ten, Suit::Diamonds));

        // Seven of Clubs is id 6
        let id: u8 = 6; // 13 * 0 + 6
        assert_eq!(Card::from(id), Card::create(Value::Seven, Suit::Clubs));

        // Queen of Hearts is id 37
        let id: u8 = 37; // 13 * 2 + 11
        assert_eq!(Card::from(id), Card::create(Value::Queen, Suit::Hearts));
    }

    #[test]
    fn test_card_to_id() {
        // King of Clubs is id 12
        let id: u8 = 12; // 13 * 0 + 12
        assert_eq!(u8::from(Card::create(Value::King, Suit::Clubs)), id);

        // Five of Diamonds is id 17
        let id: u8 = 17; // 13 * 1 + 4
        assert_eq!(u8::from(Card::create(Value::Five, Suit::Diamonds)), id);

        // Eight of Hearts is id 33
        let id: u8 = 33; // 13 * 2 + 7
        assert_eq!(u8::from(Card::create(Value::Eight, Suit::Hearts)), id);

        // Jack of Spades is id 49
        let id: u8 = 49; // 13 * 3 + 10
        assert_eq!(u8::from(Card::create(Value::Jack, Suit::Spades)), id);
    }

    #[test]
    fn test_card_to_string() {
        assert_eq!(Card::create(Value::Ace, Suit::Spades).to_string(), "A♠");
        assert_eq!(Card::create(Value::King, Suit::Hearts).to_string(), "K♥");
        assert_eq!(Card::create(Value::Queen, Suit::Diamonds).to_string(), "Q♦");
        assert_eq!(Card::create(Value::Jack, Suit::Clubs).to_string(), "J♣");
        assert_eq!(Card::create(Value::Two, Suit::Spades).to_string(), "2♠");
        assert_eq!(Card::create(Value::Three, Suit::Hearts).to_string(), "3♥");
        assert_eq!(Card::create(Value::Four, Suit::Diamonds).to_string(), "4♦");
        assert_eq!(Card::create(Value::Five, Suit::Clubs).to_string(), "5♣");
    }
}
