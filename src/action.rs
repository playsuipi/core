/// Byte parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    InvalidByte,
    InvalidAddress,
}

/// A pile address
#[derive(Debug, PartialEq, Eq)]
pub enum Address {
    Pair,      // Smart pile for auto-pairing
    Hand(u8),  // Address of a pile in your hand
    Discard,   // Smart pile for auto-discarding
    Floor(u8), // Address of a pile on the floor
}

/// The type of action
#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Passive, // Simple card movement between piles
    Active,  // Trigger a change in value or score
}

/// A single atomic Suipi action
#[derive(Debug, PartialEq, Eq)]
pub struct Action {
    pub operation: Operation,
    pub address: Address,
}

impl Action {
    /// Get an action from an operation and an address
    fn new(o: Operation, a: Address) -> Action {
        Action {
            operation: o,
            address: a,
        }
    }

    /// Get an action from a byte
    pub fn from_byte(x: u8) -> Result<Action, ParsingError> {
        if x & 0b11000000 > 0 {
            // We only use 6 bits per byte
            Err(ParsingError::InvalidByte)
        } else {
            Ok(Action::new(
                if x > 0b00100000 {
                    Operation::Active
                } else {
                    Operation::Passive
                },
                match x & 0b00011111 {
                    0 => Ok(Address::Pair),
                    1..=8 => Ok(Address::Hand((x & 0b00011111) - 1)),
                    9 => Ok(Address::Discard),
                    10..=23 => Ok(Address::Floor((x & 0b00011111) - 10)),
                    _ => Err(ParsingError::InvalidAddress),
                }?,
            ))
        }
    }
}

/// A Suipi move comprised of sequential actions
#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub actions: Vec<Action>,
}

impl Move {
    /// Get a move from a set of actions
    pub fn new(a: Vec<Action>) -> Move {
        Move { actions: a }
    }

    /// Get a move from a set of bytes
    pub fn from_bytes(xs: Vec<u8>) -> Result<Move, ParsingError> {
        Ok(Move::new(
            xs.iter()
                .map(|x| Action::from_byte(x.to_owned()))
                .collect::<Result<Vec<Action>, ParsingError>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: u8 = 32;
    const P: u8 = 0;

    #[test]
    fn test_from_bytes() {
        assert_eq!(
            Move::from_bytes(vec![P + 1]),
            Ok(Move::new(vec![Action::new(
                Operation::Passive,
                Address::Hand(0)
            ),]))
        );

        assert_eq!(
            Move::from_bytes(vec![A + 1, P + 10]),
            Ok(Move::new(vec![
                Action::new(Operation::Active, Address::Hand(0)),
                Action::new(Operation::Passive, Address::Floor(0)),
            ]))
        );

        assert_eq!(
            Move::from_bytes(vec![P + 10, A + 11, A + 12, P + 13, A + 14, P + 1]),
            Ok(Move::new(vec![
                Action::new(Operation::Passive, Address::Floor(0)),
                Action::new(Operation::Active, Address::Floor(1)),
                Action::new(Operation::Active, Address::Floor(2)),
                Action::new(Operation::Passive, Address::Floor(3)),
                Action::new(Operation::Active, Address::Floor(4)),
                Action::new(Operation::Passive, Address::Hand(0)),
            ]))
        );
    }

    #[test]
    fn test_from_bytes_error() {
        assert_eq!(
            Move::from_bytes(vec![P + 10, 64]),
            Err(ParsingError::InvalidByte)
        );

        assert_eq!(
            Move::from_bytes(vec![100, 101, 102]),
            Err(ParsingError::InvalidByte)
        );
    }
}
