//! Saving the tracker as JSON text and reading it back: the browser's storage and the backup file.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::catalog::{Exercise, ExerciseId, RepRange, seeded};
use crate::muscle::Day;
use crate::progression::MAX_REPS;
use crate::session::{Draft, Session};
use crate::tracker::Tracker;
use crate::weight::{Rack, Unit, Weight};

/// The snapshot format written by this version; a snapshot of another version is not read.
const SNAPSHOT_VERSION: u32 = 1;

/// What the tracker keeps. Seeded exercises come from the code, so only the user's own are saved.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    version: u32,
    unit: Unit,
    rack: Rack,
    weekly_goal: u32,
    custom_exercises: Vec<Exercise>,
    plan: Vec<ExerciseId>,
    sessions: Vec<Session>,
    next_day: Day,
    workout: Option<Draft>,
}

/// Why saved text could not be read.
#[derive(Debug)]
pub enum LoadError {
    /// Not a snapshot this tracker understands.
    Malformed(serde_json::Error),
    /// A snapshot of another format version.
    Version(u32),
    /// The snapshot names an exercise it does not define.
    UnknownExercise(ExerciseId),
    /// The snapshot holds a value the tracker never writes.
    Invalid(&'static str),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Malformed(error) => write!(f, "not an ironless backup: {error}"),
            LoadError::Version(version) => {
                write!(f, "backup version {version} is not {SNAPSHOT_VERSION}")
            }
            LoadError::UnknownExercise(id) => write!(f, "backup names an unknown exercise {id:?}"),
            LoadError::Invalid(what) => write!(f, "backup holds {what}"),
        }
    }
}

impl std::error::Error for LoadError {}

impl Tracker {
    /// Everything the tracker keeps, as text [`Tracker::load`] reads back.
    pub fn save(&self) -> String {
        let snapshot = Snapshot {
            version: SNAPSHOT_VERSION,
            unit: self.unit,
            rack: self.rack.clone(),
            weekly_goal: self.weekly_goal,
            custom_exercises: self.custom.clone(),
            plan: self.plan.clone(),
            sessions: self.sessions.clone(),
            next_day: self.next_day,
            workout: self.draft.clone(),
        };
        serde_json::to_string(&snapshot).expect("a snapshot holds only strings, numbers and lists")
    }

    /// The tracker [`Tracker::save`] wrote.
    pub fn load(text: &str) -> Result<Self, LoadError> {
        let snapshot: Snapshot = serde_json::from_str(text).map_err(LoadError::Malformed)?;
        if snapshot.version != SNAPSHOT_VERSION {
            return Err(LoadError::Version(snapshot.version));
        }
        let tracker = Tracker {
            unit: snapshot.unit,
            rack: snapshot.rack,
            weekly_goal: snapshot.weekly_goal.clamp(1, 7),
            seeded: seeded(),
            custom: snapshot.custom_exercises,
            plan: snapshot.plan,
            sessions: snapshot.sessions,
            next_day: snapshot.next_day,
            draft: snapshot.workout,
        };
        tracker.check_references()?;
        tracker.check_values()?;
        Ok(tracker)
    }

    fn check_references(&self) -> Result<(), LoadError> {
        match self.referenced().find(|id| self.exercise(id).is_none()) {
            Some(id) => Err(LoadError::UnknownExercise(id.clone())),
            None => Ok(()),
        }
    }

    fn check_values(&self) -> Result<(), LoadError> {
        if self.rack.weights().any(|weight| weight == Weight::ZERO) {
            return Err(LoadError::Invalid("a dumbbell weighing 0"));
        }
        if !self
            .custom
            .iter()
            .all(|exercise| valid_range(exercise.reps))
        {
            return Err(LoadError::Invalid(
                "an exercise with a rep range it cannot have",
            ));
        }
        if !self.reps_kept().all(|reps| (1..=MAX_REPS).contains(&reps)) {
            return Err(LoadError::Invalid("a set with reps it cannot have"));
        }
        Ok(())
    }

    /// The reps of every logged set and every set of the workout in progress.
    fn reps_kept(&self) -> impl Iterator<Item = u32> {
        let logged = self
            .sessions
            .iter()
            .flat_map(|session| &session.exercises)
            .flat_map(|log| log.sets.iter().map(|set| set.reps));
        let drafted = self
            .draft
            .iter()
            .flat_map(|draft| &draft.exercises)
            .flat_map(|exercise| exercise.sets.iter().map(|set| set.reps));
        logged.chain(drafted)
    }

    /// Every exercise id the plan, the workouts and the workout in progress name.
    fn referenced(&self) -> impl Iterator<Item = &ExerciseId> {
        let logged = self
            .sessions
            .iter()
            .flat_map(|session| session.exercises.iter().map(|log| &log.exercise));
        let drafted = self
            .draft
            .iter()
            .flat_map(|draft| draft.exercises.iter().map(|exercise| &exercise.exercise));
        self.plan.iter().chain(logged).chain(drafted)
    }
}

fn valid_range(reps: RepRange) -> bool {
    1 <= reps.min && reps.min <= reps.max && reps.max <= MAX_REPS
}
