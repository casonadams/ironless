//! The training days of the push / pull / legs rotation and the muscles they train.

use serde::{Deserialize, Serialize};

/// A day of the rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Day {
    /// Chest, shoulders and triceps.
    Push,
    /// Back, rear delts and biceps.
    Pull,
    /// Quads, hamstrings, glutes and calves.
    Legs,
}

impl Day {
    /// The rotation in order.
    pub const ALL: [Day; 3] = [Day::Push, Day::Pull, Day::Legs];

    /// The day that follows this one in the rotation.
    pub fn next(self) -> Day {
        match self {
            Day::Push => Day::Pull,
            Day::Pull => Day::Legs,
            Day::Legs => Day::Push,
        }
    }

    /// The day's name, as the app shows it.
    pub fn name(self) -> &'static str {
        match self {
            Day::Push => "Push",
            Day::Pull => "Pull",
            Day::Legs => "Legs",
        }
    }
}

/// A muscle group whose weekly sets are counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Muscle {
    Chest,
    Shoulders,
    Triceps,
    Back,
    Biceps,
    Quads,
    Hamstrings,
    Glutes,
    Calves,
}

impl Muscle {
    /// Every muscle, in the order the rotation trains them.
    pub const ALL: [Muscle; 9] = [
        Muscle::Chest,
        Muscle::Shoulders,
        Muscle::Triceps,
        Muscle::Back,
        Muscle::Biceps,
        Muscle::Quads,
        Muscle::Hamstrings,
        Muscle::Glutes,
        Muscle::Calves,
    ];

    /// The day an exercise for this muscle belongs to unless it says otherwise.
    pub const fn day(self) -> Day {
        match self {
            Muscle::Chest | Muscle::Shoulders | Muscle::Triceps => Day::Push,
            Muscle::Back | Muscle::Biceps => Day::Pull,
            Muscle::Quads | Muscle::Hamstrings | Muscle::Glutes | Muscle::Calves => Day::Legs,
        }
    }

    /// The muscle's name, as the app shows it.
    pub fn name(self) -> &'static str {
        match self {
            Muscle::Chest => "Chest",
            Muscle::Shoulders => "Shoulders",
            Muscle::Triceps => "Triceps",
            Muscle::Back => "Back",
            Muscle::Biceps => "Biceps",
            Muscle::Quads => "Quads",
            Muscle::Hamstrings => "Hamstrings",
            Muscle::Glutes => "Glutes",
            Muscle::Calves => "Calves",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_rotation_cycles_push_pull_legs() {
        assert_eq!(Day::Push.next(), Day::Pull);
        assert_eq!(Day::Pull.next(), Day::Legs);
        assert_eq!(Day::Legs.next(), Day::Push);
    }

    #[test]
    fn every_day_trains_some_muscle() {
        for day in Day::ALL {
            assert!(
                Muscle::ALL.iter().any(|muscle| muscle.day() == day),
                "{day:?}"
            );
        }
    }
}
