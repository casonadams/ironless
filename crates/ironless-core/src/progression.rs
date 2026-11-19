//! Double progression: add reps up to the top of the range on every set, then step up to the next owned dumbbell.

use serde::{Deserialize, Serialize};

use crate::catalog::{Exercise, Load, SETS_PER_EXERCISE};
use crate::weight::{Rack, Unit, Weight};

/// The most reps a set may log.
pub const MAX_REPS: u32 = 999;

/// One set as lifted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetLog {
    pub weight: Weight,
    pub reps: u32,
}

/// What the next workout should do for an exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Advice {
    /// No history yet: pick a starting weight.
    First,
    /// Same weight, one more rep per set up to the top of the range.
    AddReps,
    /// Every set reached the top of the range: the next owned weight, back at the bottom of the range.
    AddWeight(Weight),
    /// Every set reached the top of the range and there is nothing heavier: keep adding reps.
    NoHeavier,
}

/// The sets the next workout starts with for an exercise, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub advice: Advice,
    /// The sets to aim for.
    pub sets: Vec<SetLog>,
    /// The sets of the last workout with this exercise, empty for none.
    pub last: Vec<SetLog>,
}

/// The target for `exercise` after `last`, the sets of its latest workout, stepping weights on `rack`.
pub fn target(exercise: &Exercise, last: Option<&[SetLog]>, rack: &Rack) -> Target {
    match last.filter(|sets| !sets.is_empty()) {
        None => first_target(exercise, rack),
        Some(sets) => next_target(exercise, sets, rack),
    }
}

fn first_target(exercise: &Exercise, rack: &Rack) -> Target {
    let weight = match exercise.load {
        Load::Bodyweight => Weight::ZERO,
        Load::Dumbbells => rack.lightest().unwrap_or(Weight::ZERO),
    };
    let set = SetLog {
        weight,
        reps: exercise.reps.min,
    };
    Target {
        advice: Advice::First,
        sets: vec![set; SETS_PER_EXERCISE],
        last: Vec::new(),
    }
}

fn next_target(exercise: &Exercise, last: &[SetLog], rack: &Rack) -> Target {
    let working = last
        .iter()
        .map(|set| set.weight)
        .max()
        .unwrap_or(Weight::ZERO);
    let reps: Vec<u32> = last
        .iter()
        .filter(|set| set.weight == working)
        .map(|set| set.reps)
        .collect();
    let top = exercise.reps.max;
    let all_at_top = reps.len() >= SETS_PER_EXERCISE && reps.iter().all(|done| *done >= top);
    let heavier = (exercise.load == Load::Dumbbells)
        .then(|| rack.above(working))
        .flatten();
    let (advice, sets) = match (all_at_top, heavier) {
        (true, Some(next)) => (Advice::AddWeight(next), at_weight(next, exercise.reps.min)),
        (true, None) => (Advice::NoHeavier, one_more_rep(&reps, working, MAX_REPS)),
        (false, _) => (Advice::AddReps, one_more_rep(&reps, working, top)),
    };
    Target {
        advice,
        sets,
        last: last.to_vec(),
    }
}

fn at_weight(weight: Weight, reps: u32) -> Vec<SetLog> {
    vec![SetLog { weight, reps }; SETS_PER_EXERCISE]
}

/// The planned sets at `weight`, each one rep past the matching set of `reps` (the fewest for a set beyond them), at
/// most `cap`.
fn one_more_rep(reps: &[u32], weight: Weight, cap: u32) -> Vec<SetLog> {
    let fewest = reps.iter().copied().min().unwrap_or(0);
    (0..SETS_PER_EXERCISE.max(reps.len()))
        .map(|index| SetLog {
            weight,
            reps: (reps.get(index).copied().unwrap_or(fewest) + 1).min(cap),
        })
        .collect()
}

impl Target {
    /// A one-line hint for the workout, such as `Beat last time: 25 lb x 10, 9, 9`.
    pub fn hint(&self, exercise: &Exercise, unit: Unit) -> String {
        match self.advice {
            Advice::First if exercise.load == Load::Bodyweight => {
                format!(
                    "First time: as many clean reps as you can, aiming for {}",
                    exercise.reps
                )
            }
            Advice::First => format!(
                "First time: pick a weight you can lift {} times",
                exercise.reps
            ),
            Advice::AddReps => format!("Beat last time: {}", describe(&self.last, unit)),
            Advice::AddWeight(weight) => {
                format!(
                    "Top of the range last time: move up to {}",
                    weight.label(unit)
                )
            }
            Advice::NoHeavier => {
                format!("No heavier dumbbell: beat {}", describe(&self.last, unit))
            }
        }
    }
}

/// Sets written compactly: `25 lb x 10, 9, 9`, or `20 lb x 12, 25 lb x 8` when the weight changes.
pub fn describe(sets: &[SetLog], unit: Unit) -> String {
    let mut groups: Vec<(Weight, Vec<String>)> = Vec::new();
    for set in sets {
        match groups.last_mut() {
            Some((weight, reps)) if *weight == set.weight => reps.push(set.reps.to_string()),
            _ => groups.push((set.weight, vec![set.reps.to_string()])),
        }
    }
    let groups = groups
        .into_iter()
        .map(|(weight, reps)| format!("{} x {}", weight.set_label(unit), reps.join(", ")));
    groups.collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{ExerciseId, RepRange};
    use crate::muscle::{Day, Muscle};

    fn exercise(load: Load) -> Exercise {
        Exercise {
            id: ExerciseId::new("curl"),
            name: "Curl".to_owned(),
            day: Day::Pull,
            muscle: Muscle::Biceps,
            reps: RepRange { min: 8, max: 12 },
            load,
            position: None,
        }
    }

    fn rack(wholes: &[u32]) -> Rack {
        let mut rack = Rack::default();
        for whole in wholes {
            rack.add(Weight::whole(*whole)).expect("owned");
        }
        rack
    }

    fn sets(weight: u32, reps: &[u32]) -> Vec<SetLog> {
        reps.iter()
            .map(|reps| SetLog {
                weight: Weight::whole(weight),
                reps: *reps,
            })
            .collect()
    }

    fn mixed(sets: &[(u32, u32)]) -> Vec<SetLog> {
        sets.iter()
            .map(|(weight, reps)| SetLog {
                weight: Weight::whole(*weight),
                reps: *reps,
            })
            .collect()
    }

    #[test]
    fn double_progression_targets() {
        let owned = rack(&[15, 20, 25]);
        let cases = [
            (
                "nothing logged starts light",
                None,
                Advice::First,
                sets(15, &[8, 8, 8]),
            ),
            (
                "adds a rep per set",
                Some(sets(20, &[10, 9, 8])),
                Advice::AddReps,
                sets(20, &[11, 10, 9]),
            ),
            (
                "caps reps at the top",
                Some(sets(20, &[12, 11, 12])),
                Advice::AddReps,
                sets(20, &[12, 12, 12]),
            ),
            (
                "steps up at the top",
                Some(sets(20, &[12, 12, 13])),
                Advice::AddWeight(Weight::whole(25)),
                sets(25, &[8, 8, 8]),
            ),
            (
                "no heavier dumbbell",
                Some(sets(25, &[12, 12, 12])),
                Advice::NoHeavier,
                sets(25, &[13, 13, 13]),
            ),
            (
                "two sets is not the top",
                Some(sets(20, &[12, 12])),
                Advice::AddReps,
                sets(20, &[12, 12, 12]),
            ),
            (
                "reps at a lighter weight do not carry to the heavier one",
                Some(mixed(&[(20, 12), (25, 8), (25, 7)])),
                Advice::AddReps,
                sets(25, &[9, 8, 8]),
            ),
            (
                "only sets at the heaviest weight count toward the top",
                Some(mixed(&[(25, 12), (20, 12), (20, 12)])),
                Advice::AddReps,
                sets(25, &[12, 12, 12]),
            ),
        ];
        for (name, last, advice, expected) in cases {
            let target = target(&exercise(Load::Dumbbells), last.as_deref(), &owned);
            assert_eq!((target.advice, target.sets), (advice, expected), "{name}");
        }
    }

    #[test]
    fn bodyweight_exercises_progress_by_reps_only() {
        let push_up = exercise(Load::Bodyweight);
        let owned = rack(&[10, 20]);
        let first = target(&push_up, None, &owned);
        assert_eq!(first.sets, sets(0, &[8, 8, 8]));
        let top = target(&push_up, Some(&sets(0, &[12, 12, 12])), &owned);
        assert_eq!(
            (top.advice, top.sets),
            (Advice::NoHeavier, sets(0, &[13, 13, 13]))
        );
    }

    #[test]
    fn steps_up_from_a_weight_no_longer_owned() {
        let target = target(
            &exercise(Load::Dumbbells),
            Some(&sets(17, &[12, 12, 12])),
            &rack(&[15, 20]),
        );
        assert_eq!(target.advice, Advice::AddWeight(Weight::whole(20)));
    }

    #[test]
    fn hints_say_what_to_beat() {
        let curl = exercise(Load::Dumbbells);
        let owned = rack(&[20, 25]);
        let cases = [
            (None, "First time: pick a weight you can lift 8-12 times"),
            (
                Some(sets(20, &[10, 9, 9])),
                "Beat last time: 20 lb x 10, 9, 9",
            ),
            (
                Some(sets(20, &[12, 12, 12])),
                "Top of the range last time: move up to 25 lb",
            ),
            (
                Some(sets(25, &[12, 12, 12])),
                "No heavier dumbbell: beat 25 lb x 12, 12, 12",
            ),
        ];
        for (last, hint) in cases {
            assert_eq!(
                target(&curl, last.as_deref(), &owned).hint(&curl, Unit::Lb),
                hint
            );
        }
    }

    #[test]
    fn describes_weight_changes_and_bodyweight() {
        let mut mixed = sets(20, &[12]);
        mixed.extend(sets(25, &[8, 7]));
        assert_eq!(describe(&mixed, Unit::Kg), "20 kg x 12, 25 kg x 8, 7");
        assert_eq!(
            describe(&sets(0, &[15, 12]), Unit::Lb),
            "Bodyweight x 15, 12"
        );
    }
}
