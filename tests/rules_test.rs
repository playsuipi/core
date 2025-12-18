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

#[test]
fn test_cannot_orphan_owned_pile() {
    let mut g = setup([
        156, 69, 3, 119, 217, 73, 100, 245, 25, 0, 13, 180, 77, 217, 127, 113, 188, 61, 115, 22,
        13, 229, 255, 166, 56, 212, 40, 145, 67, 218, 143, 98,
    ]);
    apply_moves(
        &mut g,
        vec![
            "*A+D&6", "*A&3", "!7", "*A&7", "*A&5", "!6", "!2", "*A+B&8", "!4", "!5", "!1", "!1",
            "!8", "!2", "!3", "!4", "*D+F&1", "*A&1", "*E&5", "*C&2", "*C&7", "!4", "*A&2", "*B&6",
            "!8", "!5", "*C&3", "!3", "!4", "!8", "!6", "!7", "E+F+7", "*A&3", "*C&8", "*B&8",
            "*A&3", "!4",
        ],
    );
    // Attempt to pair a 10, while owning a separate 10 pile on the floor
    let res = apply(&mut g, "*B+C&6");
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), StateError::OrphanedPile.to_string());
}
