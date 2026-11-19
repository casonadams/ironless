//! The tracker: settings, the plan, the workout in progress and the finished workouts.

use std::fmt;

use crate::catalog::{Exercise, ExerciseId, Load, RepRange, seeded, starting_plan};
use crate::muscle::{Day, Muscle};
use crate::progression::{SetLog, Target, target};
use crate::session::{Draft, DraftExercise, DraftSet, Session};
use crate::weight::{Rack, RackError, Unit, Weight};

/// The longest name a user's exercise may have, in characters.
pub const MAX_NAME_CHARS: usize = 40;

/// Workouts per week the goal starts at: the five training days.
const DEFAULT_WEEKLY_GOAL: u32 = 5;

/// Why the tracker refused a change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackerError {
    /// No exercise has this id.
    UnknownExercise(String),
    /// The workout has no set at this position.
    NoSuchSet,
    /// A workout is finished only with at least one set done.
    NothingDone,
    /// Another day cannot start while a set of the workout in progress is done.
    WorkoutStarted,
    /// An exercise needs a name.
    EmptyName,
    /// An exercise name is at most [`MAX_NAME_CHARS`] characters.
    NameTooLong,
    /// Another exercise has this name.
    DuplicateName,
    /// The weight cannot go on the rack.
    Rack(RackError),
}

impl fmt::Display for TrackerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrackerError::UnknownExercise(id) => write!(f, "no exercise has the id {id:?}"),
            TrackerError::NoSuchSet => write!(f, "the workout has no such set"),
            TrackerError::NothingDone => write!(f, "mark at least one set done to finish"),
            TrackerError::WorkoutStarted => {
                write!(f, "finish this workout or undo its done sets to switch day")
            }
            TrackerError::EmptyName => write!(f, "an exercise needs a name"),
            TrackerError::NameTooLong => {
                write!(f, "an exercise name is at most {MAX_NAME_CHARS} characters")
            }
            TrackerError::DuplicateName => write!(f, "another exercise has this name"),
            TrackerError::Rack(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for TrackerError {}

/// The tracker's whole state.
#[derive(Debug, Clone, PartialEq)]
pub struct Tracker {
    pub(crate) unit: Unit,
    pub(crate) rack: Rack,
    pub(crate) weekly_goal: u32,
    pub(crate) seeded: Vec<Exercise>,
    pub(crate) custom: Vec<Exercise>,
    pub(crate) plan: Vec<ExerciseId>,
    pub(crate) sessions: Vec<Session>,
    pub(crate) next_day: Day,
    pub(crate) draft: Option<Draft>,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker {
            unit: Unit::default(),
            rack: Rack::starter(),
            weekly_goal: DEFAULT_WEEKLY_GOAL,
            seeded: seeded(),
            custom: Vec::new(),
            plan: starting_plan(),
            sessions: Vec::new(),
            next_day: Day::Push,
            draft: None,
        }
    }
}

impl Tracker {
    /// The unit weights are written in.
    pub fn unit(&self) -> Unit {
        self.unit
    }

    /// Writes every weight in `unit`; the numbers stay as they are.
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }

    /// The dumbbells owned.
    pub fn rack(&self) -> &Rack {
        &self.rack
    }

    /// Adds a dumbbell weight to the rack.
    pub fn add_weight(&mut self, weight: Weight) -> Result<(), TrackerError> {
        self.rack.add(weight).map_err(TrackerError::Rack)?;
        self.refresh_unstarted_workout();
        Ok(())
    }

    /// Removes a dumbbell weight from the rack; logged sets keep it.
    pub fn remove_weight(&mut self, weight: Weight) {
        self.rack.remove(weight);
        self.refresh_unstarted_workout();
    }

    /// Workouts a week the user aims for.
    pub fn weekly_goal(&self) -> u32 {
        self.weekly_goal
    }

    /// Sets the weekly goal, kept within 1 to 7.
    pub fn set_weekly_goal(&mut self, goal: u32) {
        self.weekly_goal = goal.clamp(1, 7);
    }

    /// Every exercise: the seeded ones, then the user's own.
    pub fn exercises(&self) -> impl Iterator<Item = &Exercise> {
        self.seeded.iter().chain(&self.custom)
    }

    /// The exercise with `id`.
    pub fn exercise(&self, id: &ExerciseId) -> Option<&Exercise> {
        self.exercises().find(|exercise| &exercise.id == id)
    }

    pub(crate) fn known(&self, id: &ExerciseId) -> Result<&Exercise, TrackerError> {
        self.exercise(id)
            .ok_or_else(|| TrackerError::UnknownExercise(id.to_string()))
    }

    /// The exercises `day`'s workouts include, in workout order.
    pub fn plan_for(&self, day: Day) -> Vec<&Exercise> {
        self.plan
            .iter()
            .filter_map(|id| self.exercise(id))
            .filter(|exercise| exercise.day == day)
            .collect()
    }

    /// Whether workouts include the exercise.
    pub fn is_planned(&self, id: &ExerciseId) -> bool {
        self.plan.contains(id)
    }

    /// Includes the exercise in its day's workouts, or leaves it out.
    pub fn toggle_planned(&mut self, id: &ExerciseId) -> Result<(), TrackerError> {
        self.known(id)?;
        match self.plan.iter().position(|planned| planned == id) {
            Some(index) => {
                self.plan.remove(index);
            }
            None => self.plan.push(id.clone()),
        }
        self.refresh_unstarted_workout();
        Ok(())
    }

    /// Adds a dumbbell exercise of the user's own for `muscle`, included in that muscle's day.
    pub fn add_exercise(&mut self, name: &str, muscle: Muscle) -> Result<ExerciseId, TrackerError> {
        let name = self.new_name(name)?;
        let id = ExerciseId::new(format!("custom-{}", self.custom.len() + 1));
        self.custom.push(Exercise {
            id: id.clone(),
            name,
            day: muscle.day(),
            muscle,
            reps: RepRange { min: 8, max: 12 },
            load: Load::Dumbbells,
        });
        self.plan.push(id.clone());
        self.refresh_unstarted_workout();
        Ok(id)
    }

    fn new_name(&self, name: &str) -> Result<String, TrackerError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(TrackerError::EmptyName);
        }
        if name.chars().count() > MAX_NAME_CHARS {
            return Err(TrackerError::NameTooLong);
        }
        let taken = self
            .exercises()
            .any(|exercise| exercise.name.eq_ignore_ascii_case(name));
        if taken {
            return Err(TrackerError::DuplicateName);
        }
        Ok(name.to_owned())
    }

    /// The day the rotation suggests next.
    pub fn suggested_day(&self) -> Day {
        self.next_day
    }

    /// The finished workouts, oldest first.
    pub fn sessions(&self) -> &[Session] {
        &self.sessions
    }

    /// The sets of the latest workout that included `id`.
    pub fn last_sets(&self, id: &ExerciseId) -> Option<&[SetLog]> {
        self.sessions
            .iter()
            .rev()
            .find_map(|session| session.sets_of(id))
    }

    /// What the next workout should do for the exercise.
    pub fn target(&self, id: &ExerciseId) -> Result<Target, TrackerError> {
        let exercise = self.known(id)?;
        Ok(target(exercise, self.last_sets(id), &self.rack))
    }

    pub(crate) fn draft_for(&self, day: Day) -> Draft {
        let exercises = self
            .plan_for(day)
            .into_iter()
            .map(|exercise| DraftExercise {
                exercise: exercise.id.clone(),
                sets: target(exercise, self.last_sets(&exercise.id), &self.rack)
                    .sets
                    .into_iter()
                    .map(|set| DraftSet {
                        weight: set.weight,
                        reps: set.reps,
                        done: false,
                    })
                    .collect(),
            });
        Draft {
            day,
            exercises: exercises.collect(),
        }
    }

    /// A workout no set of which is done follows plan and rack changes.
    fn refresh_unstarted_workout(&mut self) {
        let unstarted = self.draft.as_ref().filter(|draft| !draft.any_done());
        if let Some(day) = unstarted.map(|draft| draft.day) {
            self.draft = Some(self.draft_for(day));
        }
    }
}
