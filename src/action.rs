/// A pile address
#[derive(Debug)]
pub enum Address {
    Pair,       // Smart pile for auto-pairing
    Hand(u8),   // Address of a pile in your hand
    Discard,    // Smart pile for auto-discarding
    Floor(u16), // Address of a pile on the floor
}

/// The operation of a step
#[derive(Debug)]
pub enum Operation {
    Group, // Group with the previous pile
    Build, // Build on the previous pile
}

/// A step in an action
#[derive(Debug)]
pub struct Step {
    pub operation: Operation,
    pub address: Address,
}

/// An action in a Suipi game
#[derive(Debug)]
pub struct Action {
    pub steps: Vec<Step>,
}

impl Action {
    /// Get an action from a Suipi annotation
    pub fn from_annotation(_annotation: String) -> Action {
        Action { steps: vec![] }
    }
}
