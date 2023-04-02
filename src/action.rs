/// Annotation parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum AnnotationError {
    InvalidCharacter,
}

/// A pile address
#[derive(Debug, PartialEq, Eq)]
pub enum Address {
    Pair,      // Smart pile for auto-pairing
    Hand(u8),  // Address of a pile in your hand
    Discard,   // Smart pile for auto-discarding
    Floor(u8), // Address of a pile on the floor
}

/// The operation of a step
#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Passive, // Simple card movement between piles
    Active,  // Trigger a change in value or score
}

/// A step in an action
#[derive(Debug, PartialEq, Eq)]
pub struct Step {
    pub operation: Operation,
    pub address: Address,
}

impl Step {
    /// Get a step from an op and an addr
    fn new(o: Operation, a: Address) -> Step {
        Step {
            operation: o,
            address: a,
        }
    }
}

/// An action in a Suipi game
#[derive(Debug, PartialEq, Eq)]
pub struct Action {
    pub steps: Vec<Step>,
}

impl Action {
    /// Get an action from a vec of steps
    fn new(xs: Vec<Step>) -> Action {
        Action { steps: xs }
    }

    /// Get an action from a Suipi annotation
    pub fn from_annotation(annotation: &String) -> Result<Action, AnnotationError> {
        let mut steps: Vec<Step> = vec![];
        let mut o: Option<Operation> = Some(Operation::Passive);
        let mut a: Option<Address> = None;
        for ch in annotation.chars() {
            match ch {
                '!' => steps.push(Step::new(Operation::Passive, Address::Discard)),
                '*' => steps.push(Step::new(Operation::Active, Address::Pair)),
                '&' => o = Some(Operation::Passive),
                '+' => o = Some(Operation::Active),
                '0' => a = Some(Address::Pair),
                '1'..='8' => a = Some(Address::Hand(ch as u8 - 49)),
                '9' => a = Some(Address::Discard),
                'A'..='M' => a = Some(Address::Floor(ch as u8 - 65)),
                _ => {}
            }

            if o.is_some() && a.is_some() {
                steps.push(Step::new(o.unwrap(), a.unwrap()));
                o = None;
                a = None;
            }
        }

        Ok(Action::new(steps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discard_smart_pile() {
        let a = Action::from_annotation(&String::from("!1"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Passive, Address::Discard),
                Step::new(Operation::Passive, Address::Hand(0)),
            ])),
        );

        let a = Action::from_annotation(&String::from("!8"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Passive, Address::Discard),
                Step::new(Operation::Passive, Address::Hand(7)),
            ])),
        );

        // Valid syntax, invalid move
        let a = Action::from_annotation(&String::from("!A"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Passive, Address::Discard),
                Step::new(Operation::Passive, Address::Floor(0)),
            ])),
        );

        // Invalid syntax, still parsable?
        let a = Action::from_annotation(&String::from("!!"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Passive, Address::Discard),
                Step::new(Operation::Passive, Address::Discard),
            ])),
        );
    }

    #[test]
    fn test_pair_smart_pile() {
        let a = Action::from_annotation(&String::from("*1&A"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Active, Address::Pair),
                Step::new(Operation::Passive, Address::Hand(0)),
                Step::new(Operation::Passive, Address::Floor(0)),
            ])),
        );

        let a = Action::from_annotation(&String::from("*5&B+C"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Active, Address::Pair),
                Step::new(Operation::Passive, Address::Hand(4)),
                Step::new(Operation::Passive, Address::Floor(1)),
                Step::new(Operation::Active, Address::Floor(2)),
            ])),
        );

        // Valid syntax, invalid move
        let a = Action::from_annotation(&String::from("*A+B+C"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Active, Address::Pair),
                Step::new(Operation::Passive, Address::Floor(0)),
                Step::new(Operation::Active, Address::Floor(1)),
                Step::new(Operation::Active, Address::Floor(2)),
            ])),
        );

        // Not cool...
        let a = Action::from_annotation(&String::from("*!*!"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Active, Address::Pair),
                Step::new(Operation::Passive, Address::Discard),
                Step::new(Operation::Active, Address::Pair),
                Step::new(Operation::Passive, Address::Discard),
            ])),
        );
    }

    #[test]
    fn test_standard_piles() {
        let a = Action::from_annotation(&String::from("A+B+C&D+E&1"));
        assert_eq!(
            a,
            Ok(Action::new(vec![
                Step::new(Operation::Passive, Address::Floor(0)),
                Step::new(Operation::Active, Address::Floor(1)),
                Step::new(Operation::Active, Address::Floor(2)),
                Step::new(Operation::Passive, Address::Floor(3)),
                Step::new(Operation::Active, Address::Floor(4)),
                Step::new(Operation::Passive, Address::Hand(0)),
            ])),
        );
    }
}
