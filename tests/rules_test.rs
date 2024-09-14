use playsuipi_core::action::MoveError;
use playsuipi_core::pile::PileError;
use playsuipi_core::state::StateError;

#[allow(dead_code)]
mod common;
use common::*;

#[test]
fn test_cannot_build_over_ten() {
    let mut g = setup_default();
    let res = apply(&mut g, "*A+B+C&2");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidPile(PileError::BuildHigherThanTen).to_string()
    );
}

#[test]
fn test_cannot_build_same_values() {
    let mut g = setup_default();
    let res = apply(&mut g, "C+3");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidPile(PileError::BuildEqualValues).to_string()
    );
}

#[test]
fn test_cannot_group_different_values() {
    let mut g = setup_default();
    let res = apply(&mut g, "A+C&6");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidPile(PileError::GroupDifferentValues).to_string()
    );
}

#[test]
fn test_cannot_group_two_singles() {
    let mut g = setup_default();
    let res = apply(&mut g, "C&3");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidPile(PileError::GroupTwoSingles).to_string()
    );
}

#[test]
fn test_cannot_pair_without_a_single() {
    let mut g = setup([
        84, 203, 45, 46, 121, 160, 195, 38, 74, 65, 246, 230, 155, 184, 39, 49, 159, 197, 58, 163,
        223, 210, 157, 16, 155, 11, 149, 244, 232, 186, 101, 69,
    ]);
    let res = apply(&mut g, "*B+C&D+3");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidPile(PileError::InvalidPairArg).to_string()
    );
}

#[test]
fn test_cannot_pair_different_values() {
    let mut g = setup_default();
    let res = apply(&mut g, "*A&3");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidPile(PileError::PairDifferentValues).to_string()
    );
}

#[test]
fn test_cannot_duplicate_addresses() {
    let mut g = setup_default();
    let res = apply(&mut g, "*A+A&6");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidMove(MoveError::DuplicateAddress).to_string()
    );
}

#[test]
fn test_cannot_use_multiple_hand_address() {
    let mut g = setup_default();
    let res = apply(&mut g, "A+C+3&B+4&8");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidMove(MoveError::InvalidHandAddressCount).to_string()
    );
}

#[test]
fn test_cannot_skip_hand_address() {
    let mut g = setup_default();
    let res = apply(&mut g, "A+C");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidMove(MoveError::InvalidHandAddressCount).to_string()
    );
}

#[test]
fn test_cannot_start_pair_with_hand_address() {
    let mut g = setup_default();
    let res = apply(&mut g, "*5&C");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::InvalidMove(MoveError::InvalidHandAddressPosition).to_string()
    );
}

#[test]
fn test_cannot_own_multiple_piles() {
    let mut g = setup_default();
    apply_moves(&mut g, vec!["D&B+4", "*A&2"]);
    let res = apply(&mut g, "A+1");
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), StateError::OwnTooManyPiles.to_string());
}

#[test]
fn test_cannot_build_piles_you_cannot_pair() {
    let mut g = setup_default();
    let res = apply(&mut g, "C+8");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::UnpairablePileValue.to_string()
    );
}

#[test]
fn test_floor_must_be_unique() {
    let mut g = setup_default();
    let res = apply(&mut g, "A+C+1");
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap(),
        StateError::DuplicateFloorValue.to_string()
    );
}
