//! Dumbbell weights, the unit they are in, and the rack of dumbbells the user owns.

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Bound;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// The heaviest weight accepted, in tenths: 500.0 of the unit.
const MAX_TENTHS: u32 = 5000;

/// A weight in tenths of the unit: `125` is 12.5. Zero is no added weight.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Weight(u32);

impl Weight {
    /// No added weight: a bodyweight set.
    pub const ZERO: Weight = Weight(0);

    /// The weight `tenths` tenths of the unit.
    pub fn from_tenths(tenths: u32) -> Weight {
        Weight(tenths)
    }

    /// The weight `whole` units.
    pub fn whole(whole: u32) -> Weight {
        Weight(whole * 10)
    }

    /// The weight in tenths of the unit.
    pub fn tenths(self) -> u32 {
        self.0
    }

    /// The weight with its unit, such as `12.5 lb`.
    pub fn label(self, unit: Unit) -> String {
        format!("{self} {}", unit.label())
    }

    /// How a set's weight reads: its label, or `Bodyweight` for no added weight.
    pub fn set_label(self, unit: Unit) -> String {
        if self == Weight::ZERO {
            "Bodyweight".to_owned()
        } else {
            self.label(unit)
        }
    }
}

/// Text that is not a weight from 0 to 500 with at most one decimal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidWeight(pub String);

impl fmt::Display for InvalidWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "not a weight from 0 to 500 with at most one decimal: {:?}",
            self.0
        )
    }
}

impl std::error::Error for InvalidWeight {}

impl FromStr for Weight {
    type Err = InvalidWeight;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let trimmed = text.trim();
        let (whole, tenth) = trimmed.split_once('.').unwrap_or((trimmed, "0"));
        let tenths = parse_digits(whole)
            .zip(parse_tenth(tenth))
            .and_then(|(whole, tenth)| whole.checked_mul(10)?.checked_add(tenth))
            .filter(|tenths| *tenths <= MAX_TENTHS);
        tenths
            .map(Weight)
            .ok_or_else(|| InvalidWeight(text.to_owned()))
    }
}

fn parse_digits(text: &str) -> Option<u32> {
    let digits = !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());
    digits.then(|| text.parse().ok()).flatten()
}

fn parse_tenth(text: &str) -> Option<u32> {
    (text.len() == 1).then(|| parse_digits(text)).flatten()
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (whole, tenth) = (self.0 / 10, self.0 % 10);
        if tenth == 0 {
            write!(f, "{whole}")
        } else {
            write!(f, "{whole}.{tenth}")
        }
    }
}

/// The unit every weight is written in. It labels weights and converts none.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Unit {
    #[default]
    Lb,
    Kg,
}

impl Unit {
    /// `lb` or `kg`.
    pub fn label(self) -> &'static str {
        match self {
            Unit::Lb => "lb",
            Unit::Kg => "kg",
        }
    }
}

/// Why a weight cannot go on the rack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RackError {
    /// A dumbbell weighs something.
    Zero,
}

impl fmt::Display for RackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RackError::Zero => write!(f, "a dumbbell must weigh more than 0"),
        }
    }
}

impl std::error::Error for RackError {}

/// The dumbbell weights the user owns, lightest first.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rack(BTreeSet<Weight>);

impl Rack {
    /// A common home set: 5 to 50 by 5.
    pub fn starter() -> Rack {
        Rack((1..=10).map(|step| Weight::whole(step * 5)).collect())
    }

    /// The owned weights, lightest first.
    pub fn weights(&self) -> impl Iterator<Item = Weight> + '_ {
        self.0.iter().copied()
    }

    /// Adds a weight; adding one already owned changes nothing.
    pub fn add(&mut self, weight: Weight) -> Result<(), RackError> {
        if weight == Weight::ZERO {
            return Err(RackError::Zero);
        }
        self.0.insert(weight);
        Ok(())
    }

    /// Removes a weight, if owned.
    pub fn remove(&mut self, weight: Weight) {
        self.0.remove(&weight);
    }

    /// The lightest owned weight heavier than `weight`.
    pub fn above(&self, weight: Weight) -> Option<Weight> {
        self.0
            .range((Bound::Excluded(weight), Bound::Unbounded))
            .next()
            .copied()
    }

    /// The heaviest owned weight lighter than `weight`.
    pub fn below(&self, weight: Weight) -> Option<Weight> {
        self.0.range(..weight).next_back().copied()
    }

    /// The lightest owned weight.
    pub fn lightest(&self) -> Option<Weight> {
        self.0.first().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_whole_and_tenth_weights() {
        let cases = [
            ("25", 250),
            ("12.5", 125),
            (" 7.5 ", 75),
            ("0", 0),
            ("500", 5000),
            ("3.0", 30),
        ];
        for (text, tenths) in cases {
            assert_eq!(
                text.parse::<Weight>(),
                Ok(Weight::from_tenths(tenths)),
                "{text}"
            );
        }
    }

    #[test]
    fn rejects_text_that_is_not_a_weight() {
        for text in [
            "",
            "abc",
            "-5",
            "12.25",
            "12.",
            ".5",
            "500.1",
            "1e3",
            "99999999999",
        ] {
            assert!(text.parse::<Weight>().is_err(), "{text} parsed");
        }
    }

    #[test]
    fn labels_drop_a_zero_tenth() {
        assert_eq!(Weight::whole(25).label(Unit::Lb), "25 lb");
        assert_eq!(Weight::from_tenths(125).label(Unit::Kg), "12.5 kg");
        assert_eq!(Weight::ZERO.set_label(Unit::Kg), "Bodyweight");
        assert_eq!(Weight::whole(5).set_label(Unit::Kg), "5 kg");
    }

    #[test]
    fn the_rack_steps_between_owned_weights() {
        let mut rack = Rack::default();
        for whole in [10, 20, 25] {
            rack.add(Weight::whole(whole)).expect("owned");
        }
        let cases = [
            (Weight::whole(10), Some(Weight::whole(20)), None),
            (
                Weight::whole(15),
                Some(Weight::whole(20)),
                Some(Weight::whole(10)),
            ),
            (Weight::whole(25), None, Some(Weight::whole(20))),
            (Weight::ZERO, Some(Weight::whole(10)), None),
        ];
        for (weight, above, below) in cases {
            assert_eq!(rack.above(weight), above, "above {weight}");
            assert_eq!(rack.below(weight), below, "below {weight}");
        }
    }

    #[test]
    fn the_rack_holds_no_zero_and_no_duplicates() {
        let mut rack = Rack::starter();
        assert_eq!(rack.add(Weight::ZERO), Err(RackError::Zero));
        rack.add(Weight::whole(5)).expect("owned");
        assert_eq!(rack.weights().count(), 10);
        rack.remove(Weight::whole(5));
        assert_eq!(rack.lightest(), Some(Weight::whole(10)));
    }
}
