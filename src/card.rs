use std::fmt;


/// The number of unique card values
const VALUE_COUNT: u8 = 13;

/// Card identification errors
#[derive(Debug, PartialEq)]
pub enum IdError {
    InvalidSuitId,
    InvalidValueId,
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
    fn from_id(id: u8) -> Result<Suit, IdError> {
        match id {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err(IdError::InvalidSuitId),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}


// =======================
// == Suipi Card Values ==
// =======================

/// Suipi playing card values
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Value {
    /// Convert a value to a string literal
    fn as_string(&self) -> &str {
        match self {
            Value::Ace => "A",
            Value::King => "K",
            Value::Queen => "Q",
            Value::Jack => "J",
            Value::Ten => "10",
            Value::Nine => "9",
            Value::Eight => "8",
            Value::Seven => "7",
            Value::Six => "6",
            Value::Five => "5",
            Value::Four => "4",
            Value::Three => "3",
            Value::Two => "2",
        }
    }

    /// Get a value from its id
    fn from_id(id: u8) -> Result<Value, IdError> {
        match id {
            0 => Ok(Value::Ace),
            1 => Ok(Value::King),
            2 => Ok(Value::Queen),
            3 => Ok(Value::Jack),
            4 => Ok(Value::Ten),
            5 => Ok(Value::Nine),
            6 => Ok(Value::Eight),
            7 => Ok(Value::Seven),
            8 => Ok(Value::Six),
            9 => Ok(Value::Five),
            10 => Ok(Value::Four),
            11 => Ok(Value::Three),
            12 => Ok(Value::Two),
            _ => Err(IdError::InvalidValueId),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}


// =================
// == Suipi Cards ==
// =================

/// Suipi playing card
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    /// Get a new Card from a value and a suit
    pub fn new(v: Value, s: Suit) -> Card {
        Card { value: v, suit: s }
    }

    /// Get the card id
    pub fn to_id(&self) -> u8 {
        VALUE_COUNT * (self.suit as u8) + (self.value as u8)
    }

    /// Get a card from its id
    pub fn from_id(id: u8) -> Result<Card, IdError> {
        Ok(Card { value: Value::from_id(id % VALUE_COUNT)?, suit: Suit::from_id(id / 13)? })
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

        // Two of Spades is id 51
        let id: u8 = 51; // 13 * 3 + 12
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Two, Suit::Spades)));

        // Ten of Diamonds is id 17
        let id: u8 = 17; // 13 * 1 + 4
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Ten, Suit::Diamonds)));

        // Seven of Clubs is id 7
        let id: u8 = 7; // 13 * 0 + 7
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Seven, Suit::Clubs)));

        // Queen of Hearts is id 28
        let id: u8 = 28; // 13 * 2 + 2
        assert_eq!(Card::from_id(id), Ok(Card::new(Value::Queen, Suit::Hearts)));
    }

    #[test]
    fn test_card_to_id() {
        // King of Clubs is id 1
        let id: u8 = 1; // 13 * 0 + 1
        assert_eq!(Card::new(Value::King, Suit::Clubs).to_id(), id);

        // Five of Diamonds is id 22
        let id: u8 = 22; // 13 * 1 + 9
        assert_eq!(Card::new(Value::Five, Suit::Diamonds).to_id(), id);

        // Eight of Hearts is id 32
        let id: u8 = 32; // 13 * 2 + 6
        assert_eq!(Card::new(Value::Eight, Suit::Hearts).to_id(), id);

        // Jack of Spades is id 1
        let id: u8 = 42; // 13 * 3 + 3
        assert_eq!(Card::new(Value::Jack, Suit::Spades).to_id(), id);
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
