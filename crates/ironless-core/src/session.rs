//! Finished workouts, and the workout in progress.

use serde::{Deserialize, Serialize};

use crate::catalog::ExerciseId;
use crate::date::Date;
use crate::muscle::Day;
use crate::progression::SetLog;
use crate::weight::Weight;

/// The sets lifted for one exercise in a workout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseLog {
    pub exercise: ExerciseId,
    pub sets: Vec<SetLog>,
}

/// A finished workout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub date: Date,
    pub day: Day,
    pub exercises: Vec<ExerciseLog>,
}

impl Session {
    /// The sets this workout logged for `exercise`, if any.
    pub fn sets_of(&self, exercise: &ExerciseId) -> Option<&[SetLog]> {
        self.exercises
            .iter()
            .find(|log| &log.exercise == exercise)
            .map(|log| log.sets.as_slice())
    }
}

/// A set of the workout in progress: prefilled from the target, marked done once lifted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftSet {
    pub weight: Weight,
    pub reps: u32,
    pub done: bool,
}

/// An exercise of the workout in progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftExercise {
    pub exercise: ExerciseId,
    pub sets: Vec<DraftSet>,
}

/// The workout in progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Draft {
    pub day: Day,
    pub exercises: Vec<DraftExercise>,
}

/// A set of the workout in progress, by position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetRef {
    /// The exercise's index in the workout.
    pub exercise: usize,
    /// The set's index in the exercise.
    pub set: usize,
}

impl Draft {
    /// Whether any set is marked done.
    pub fn any_done(&self) -> bool {
        self.exercises
            .iter()
            .any(|exercise| exercise.sets.iter().any(|set| set.done))
    }

    /// The set at `at`, if there is one.
    pub fn set_mut(&mut self, at: SetRef) -> Option<&mut DraftSet> {
        self.exercises.get_mut(at.exercise)?.sets.get_mut(at.set)
    }

    /// The workout as finished: the done sets of each exercise that has any.
    pub fn finish(&self, date: Date) -> Session {
        let exercises = self.exercises.iter().filter_map(|exercise| {
            let sets: Vec<SetLog> = exercise
                .sets
                .iter()
                .filter(|set| set.done)
                .map(|set| SetLog {
                    weight: set.weight,
                    reps: set.reps,
                })
                .collect();
            (!sets.is_empty()).then(|| ExerciseLog {
                exercise: exercise.exercise.clone(),
                sets,
            })
        });
        Session {
            date,
            day: self.day,
            exercises: exercises.collect(),
        }
    }
}
