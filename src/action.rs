/// Byte parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum ParsingError {
    InvalidByte,
    InvalidAddress,
    InvalidOperationCharacter,
    InvalidAddressCharacter,
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

/// A single atomic action
#[derive(Debug, PartialEq, Eq)]
pub struct Action {
    pub operation: Operation,
    pub address: Address,
}

impl Action {
    /// Get an action from an operation and an address
    pub fn new(o: Operation, a: Address) -> Action {
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

/// A move comprised of sequential actions
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

/// An annotation representing a move
pub struct Annotation {
    pub value: String,
}

impl Annotation {
    /// Get an Annotation from a string
    pub fn new(v: String) -> Annotation {
        Annotation { value: v }
    }

    /// Get the value as a vector of bytes
    fn bytes(&self) -> Vec<u8> {
        match self.value.as_bytes()[0] as char {
            '!' | '*' => self.value.as_bytes().to_vec(),
            _ => [['!' as u8].as_slice(), self.value.as_bytes()].concat(),
        }
    }

    /// Convert an annotation to action bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, ParsingError> {
        self.bytes()
            .windows(2)
            .step_by(2)
            .map(|x| {
                Ok(match x[0] as char {
                    '!' | '&' => Ok(0),
                    '*' | '+' => Ok(32),
                    _ => Err(ParsingError::InvalidOperationCharacter),
                }? + match x[1] as char {
                    '0'..='9' => Ok(x[1] - '0' as u8),
                    'A'..='M' => Ok(x[1] - 'A' as u8 + 10),
                    _ => Err(ParsingError::InvalidAddressCharacter),
                }?)
            })
            .collect::<Result<Vec<u8>, ParsingError>>()
    }

    /// Convert an annotation to a move
    pub fn to_move(&self) -> Result<Move, ParsingError> {
        Move::from_bytes(self.to_bytes()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: u8 = 32;
    const P: u8 = 0;

    #[test]
    fn test_move_from_bytes() {
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
    fn test_move_from_bytes_error() {
        assert_eq!(
            Move::from_bytes(vec![P + 10, 64]),
            Err(ParsingError::InvalidByte)
        );

        assert_eq!(
            Move::from_bytes(vec![100, 101, 102]),
            Err(ParsingError::InvalidByte)
        );
    }

    #[test]
    fn test_annotation_to_bytes() {
        assert_eq!(
            Annotation::new(String::from("!1")).to_bytes(),
            Ok(vec![P + 1])
        );

        assert_eq!(
            Annotation::new(String::from("*1&A")).to_bytes(),
            Ok(vec![A + 1, P + 10])
        );

        assert_eq!(
            Annotation::new(String::from("A+B+C&D+E&1")).to_bytes(),
            Ok(vec![P + 10, A + 11, A + 12, P + 13, A + 14, P + 1])
        );
    }

    #[test]
    fn test_annotation_to_bytes_error() {
        assert_eq!(
            Annotation::new(String::from("!1A1")).to_bytes(),
            Err(ParsingError::InvalidOperationCharacter),
        );

        assert_eq!(
            Annotation::new(String::from("!1&!")).to_bytes(),
            Err(ParsingError::InvalidAddressCharacter),
        );

        assert_eq!(
            Annotation::new(String::from("?")).to_bytes(),
            Err(ParsingError::InvalidAddressCharacter),
        );
    }

    #[test]
    fn test_annotation_to_move() {
        assert_eq!(
            Annotation::new(String::from("!1")).to_move(),
            Ok(Move::new(vec![Action::new(
                Operation::Passive,
                Address::Hand(0)
            )]))
        );

        assert_eq!(
            Annotation::new(String::from("*1&A")).to_move(),
            Ok(Move::new(vec![
                Action::new(Operation::Active, Address::Hand(0)),
                Action::new(Operation::Passive, Address::Floor(0)),
            ]))
        );

        assert_eq!(
            Annotation::new(String::from("A+B+C&D+E&1")).to_move(),
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
}
