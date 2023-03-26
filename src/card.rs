use std::fmt;

/// The number of unique card values
const VALUE_COUNT: u8 = 13;

/// Card identity errors
#[derive(Debug, PartialEq)]
pub enum CardError {
    InvalidValue,
    InvalidSuit,
}

// =======================
// == Suipi Card Values ==
// =======================

/// Suipi playing card values
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    /// Convert a value to a string literal
    fn as_string(&self) -> &str {
        match self {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        }
    }

    /// Get a value from its id
    pub fn from_id(id: u8) -> Result<Value, CardError> {
        match id {
            0 => Ok(Value::Ace),
            1 => Ok(Value::Two),
            2 => Ok(Value::Three),
            3 => Ok(Value::Four),
            4 => Ok(Value::Five),
            5 => Ok(Value::Six),
            6 => Ok(Value::Seven),
            7 => Ok(Value::Eight),
            8 => Ok(Value::Nine),
            9 => Ok(Value::Ten),
            10 => Ok(Value::Jack),
            11 => Ok(Value::Queen),
            12 => Ok(Value::King),
            _ => Err(CardError::InvalidValue),
        }
    }

    /// Get the value id
    pub fn id(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

// ======================
// == Suipi Card Suits ==
// ======================

/// Suipi playing card suits
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    /// Convert a suit to a string literal
    fn as_string(&self) -> &str {
        match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        }
    }

    /// Get a suit from its id
    pub fn from_id(id: u8) -> Result<Suit, CardError> {
        match id {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err(CardError::InvalidSuit),
        }
    }

    /// Get the suit id
    pub fn id(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

// =================
// == Suipi Cards ==
// =================

/// Suipi playing card
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    /// Get a new Card from a value and a suit
    pub fn new(v: Value, s: Suit) -> Card {
        Card { value: v, suit: s }
    }

    /// Get a card from its id
    pub fn from_id(id: u8) -> Result<Card, CardError> {
        Ok(Card {
            value: Value::from_id(id % VALUE_COUNT)?,
            suit: Suit::from_id(id / 13)?,
        })
    }

    /// Get the card id
    pub fn id(&self) -> u8 {
        VALUE_COUNT * self.suit.id() + self.value.id()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_id() {
        // Ace of Spades is id 39
        let id: u8 = 39; // 13 * 3 + 0
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Ace, Suit::Spades)));

        // Two of Spades is id 40
        let id: u8 = 40; // 13 * 3 + 1
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Two, Suit::Spades)));

        // Ten of Diamonds is id 22
        let id: u8 = 22; // 13 * 1 + 9
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Ten, Suit::Diamonds)));

        // Seven of Clubs is id 6
        let id: u8 = 6; // 13 * 0 + 6
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Seven, Suit::Clubs)));

        // Queen of Hearts is id 37
        let id: u8 = 37; // 13 * 2 + 11
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Queen, Suit::Hearts)));
    }

    #[test]
    fn test_card_to_id() {
        // King of Clubs is id 12
        let id: u8 = 12; // 13 * 0 + 12
        assert_eq!(Card::new(Value::King, Suit::Clubs).id(), id);

        // Five of Diamonds is id 17
        let id: u8 = 17; // 13 * 1 + 4
        assert_eq!(Card::new(Value::Five, Suit::Diamonds).id(), id);

        // Eight of Hearts is id 33
        let id: u8 = 33; // 13 * 2 + 7
        assert_eq!(Card::new(Value::Eight, Suit::Hearts).id(), id);

        // Jack of Spades is id 49
        let id: u8 = 49; // 13 * 3 + 10
        assert_eq!(Card::new(Value::Jack, Suit::Spades).id(), id);
    }

    #[test]
    fn test_card_to_string() {
        assert_eq!(Card::new(Value::Ace, Suit::Spades).to_string(), "A♠");
        assert_eq!(Card::new(Value::King, Suit::Hearts).to_string(), "K♥");
        assert_eq!(Card::new(Value::Queen, Suit::Diamonds).to_string(), "Q♦");
        assert_eq!(Card::new(Value::Jack, Suit::Clubs).to_string(), "J♣");
        assert_eq!(Card::new(Value::Two, Suit::Spades).to_string(), "2♠");
        assert_eq!(Card::new(Value::Three, Suit::Hearts).to_string(), "3♥");
        assert_eq!(Card::new(Value::Four, Suit::Diamonds).to_string(), "4♦");
        assert_eq!(Card::new(Value::Five, Suit::Clubs).to_string(), "5♣");
    }
}
