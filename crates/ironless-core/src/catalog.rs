//! Exercises: the seeded floor and standing dumbbell moves, and the user's own.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::muscle::{Day, Muscle};

/// The sets each exercise of a workout starts with.
pub const SETS_PER_EXERCISE: usize = 3;

/// The reps a set aims for: add reps up to `max`, then add weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepRange {
    pub min: u32,
    pub max: u32,
}

impl fmt::Display for RepRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

/// What an exercise is loaded with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Load {
    /// Dumbbells from the rack.
    Dumbbells,
    /// The body alone; progress comes from reps.
    Bodyweight,
}

/// How the body is placed for an exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Position {
    /// On the floor: on the back, or face down for a push-up.
    Floor,
    /// Upright on the feet.
    Standing,
    /// Hinged forward at the hips, or braced on a knee.
    BentOver,
}

impl Position {
    /// The position as the app shows it.
    pub fn name(self) -> &'static str {
        match self {
            Position::Floor => "On the floor",
            Position::Standing => "Standing",
            Position::BentOver => "Bent over",
        }
    }
}

/// The stable name of an exercise in plans, logs and saved data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExerciseId(String);

impl ExerciseId {
    /// The id `id`.
    pub fn new(id: impl Into<String>) -> ExerciseId {
        ExerciseId(id.into())
    }

    /// The id as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExerciseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An exercise the plan can include.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    pub id: ExerciseId,
    pub name: String,
    pub day: Day,
    pub muscle: Muscle,
    pub reps: RepRange,
    pub load: Load,
    /// How the body is placed; the user's own exercises do not say.
    #[serde(default)]
    pub position: Option<Position>,
}

/// A seeded exercise and whether the starting plan includes it.
struct Seed {
    id: &'static str,
    name: &'static str,
    day: Day,
    muscle: Muscle,
    reps: (u32, u32),
    load: Load,
    position: Position,
    planned: bool,
}

/// A planned standing dumbbell exercise for `muscle` on that muscle's day, 8 to 12 reps.
const fn seed(id: &'static str, name: &'static str, muscle: Muscle) -> Seed {
    Seed {
        id,
        name,
        day: muscle.day(),
        muscle,
        reps: (8, 12),
        load: Load::Dumbbells,
        position: Position::Standing,
        planned: true,
    }
}

impl Seed {
    /// Done on the floor.
    const fn floor(self) -> Seed {
        Seed {
            position: Position::Floor,
            ..self
        }
    }

    /// Done bent over.
    const fn bent_over(self) -> Seed {
        Seed {
            position: Position::BentOver,
            ..self
        }
    }
    /// Aims for `min` to `max` reps.
    const fn reps(self, min: u32, max: u32) -> Seed {
        Seed {
            reps: (min, max),
            ..self
        }
    }

    /// An isolation move: 10 to 15 reps.
    const fn isolation(self) -> Seed {
        self.reps(10, 15)
    }

    /// Loaded by the body alone.
    const fn bodyweight(self) -> Seed {
        Seed {
            load: Load::Bodyweight,
            ..self
        }
    }

    /// Trained on `day` instead of its muscle's day.
    const fn on(self, day: Day) -> Seed {
        Seed { day, ..self }
    }

    /// Left out of the starting plan: a swap the plan offers.
    const fn alternate(self) -> Seed {
        Seed {
            planned: false,
            ..self
        }
    }
}

const SEEDS: [Seed; 22] = [
    seed("floor-press", "Floor press", Muscle::Chest).floor(),
    seed("floor-fly", "Floor fly", Muscle::Chest)
        .isolation()
        .floor(),
    seed(
        "overhead-press",
        "Standing overhead press",
        Muscle::Shoulders,
    ),
    seed("lateral-raise", "Lateral raise", Muscle::Shoulders).isolation(),
    seed(
        "lying-extension",
        "Lying triceps extension",
        Muscle::Triceps,
    )
    .isolation()
    .floor(),
    seed(
        "overhead-extension",
        "Overhead triceps extension",
        Muscle::Triceps,
    )
    .isolation(),
    seed("push-up", "Push-up", Muscle::Chest)
        .bodyweight()
        .reps(8, 20)
        .floor()
        .alternate(),
    seed("kickback", "Triceps kickback", Muscle::Triceps)
        .isolation()
        .bent_over()
        .alternate(),
    seed("bent-over-row", "Bent-over row", Muscle::Back).bent_over(),
    seed("single-arm-row", "Single-arm row", Muscle::Back).bent_over(),
    seed("floor-pullover", "Floor pullover", Muscle::Back)
        .isolation()
        .floor(),
    seed("rear-delt-fly", "Rear delt fly", Muscle::Shoulders)
        .isolation()
        .bent_over()
        .on(Day::Pull),
    seed("curl", "Curl", Muscle::Biceps),
    seed("hammer-curl", "Hammer curl", Muscle::Biceps),
    seed("cross-body-curl", "Cross-body curl", Muscle::Biceps).alternate(),
    seed("goblet-squat", "Goblet squat", Muscle::Quads),
    seed("split-squat", "Split squat", Muscle::Quads),
    seed("romanian-deadlift", "Romanian deadlift", Muscle::Hamstrings),
    seed("hip-thrust", "Floor hip thrust", Muscle::Glutes).floor(),
    seed("calf-raise", "Standing calf raise", Muscle::Calves).reps(12, 20),
    seed("reverse-lunge", "Reverse lunge", Muscle::Quads).alternate(),
    seed(
        "single-leg-rdl",
        "Single-leg Romanian deadlift",
        Muscle::Hamstrings,
    )
    .alternate(),
];

impl Seed {
    fn exercise(&self) -> Exercise {
        Exercise {
            id: ExerciseId::new(self.id),
            name: self.name.to_owned(),
            day: self.day,
            muscle: self.muscle,
            reps: RepRange {
                min: self.reps.0,
                max: self.reps.1,
            },
            load: self.load,
            position: Some(self.position),
        }
    }
}

/// Every seeded exercise, grouped by day in rotation order.
pub fn seeded() -> Vec<Exercise> {
    SEEDS.iter().map(Seed::exercise).collect()
}

/// The ids the starting plan includes, in workout order.
pub fn starting_plan() -> Vec<ExerciseId> {
    SEEDS
        .iter()
        .filter(|seed| seed.planned)
        .map(|seed| ExerciseId::new(seed.id))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn seeded_ids_are_unique() {
        let ids: HashSet<_> = seeded().into_iter().map(|exercise| exercise.id).collect();
        assert_eq!(ids.len(), SEEDS.len());
    }

    #[test]
    fn the_starting_plan_trains_every_muscle_with_dumbbells() {
        let exercises = seeded();
        let planned: Vec<_> = starting_plan()
            .into_iter()
            .map(|id| {
                exercises
                    .iter()
                    .find(|exercise| exercise.id == id)
                    .expect("seeded")
            })
            .collect();
        for muscle in Muscle::ALL {
            assert!(
                planned.iter().any(|exercise| exercise.muscle == muscle),
                "{muscle:?}"
            );
        }
        assert!(
            planned
                .iter()
                .all(|exercise| exercise.load == Load::Dumbbells)
        );
    }

    #[test]
    fn seeded_exercises_say_how_the_body_is_placed() {
        let exercises = seeded();
        let position = |id: &str| {
            exercises
                .iter()
                .find(|exercise| exercise.id.as_str() == id)
                .and_then(|exercise| exercise.position)
        };
        let cases = [
            ("floor-press", Position::Floor),
            ("push-up", Position::Floor),
            ("hip-thrust", Position::Floor),
            ("bent-over-row", Position::BentOver),
            ("kickback", Position::BentOver),
            ("rear-delt-fly", Position::BentOver),
            ("overhead-press", Position::Standing),
            ("goblet-squat", Position::Standing),
        ];
        for (id, expected) in cases {
            assert_eq!(position(id), Some(expected), "{id}");
        }
        assert!(exercises.iter().all(|exercise| exercise.position.is_some()));
    }

    #[test]
    fn rear_delt_flies_are_on_pull_day() {
        let fly = seeded()
            .into_iter()
            .find(|e| e.id.as_str() == "rear-delt-fly")
            .expect("seeded");
        assert_eq!((fly.day, fly.muscle), (Day::Pull, Muscle::Shoulders));
    }
}
