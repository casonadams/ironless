//! The tracker's state as JS objects, built from the core's types.

use ironless_core as core;
use nsh_core::prelude::*;

use crate::kinds::{Day, Muscle, Unit};

/// A set of the workout in progress.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct WorkoutSet {
    /// The weight with its unit (`25 lb`), or `Bodyweight`.
    pub weight: String,
    pub reps: u32,
    pub done: bool,
}

/// An exercise of the workout in progress.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct WorkoutExercise {
    pub id: String,
    pub name: String,
    pub muscle: Muscle,
    /// What to aim for, from the last workout with it.
    pub hint: String,
    /// Whether its weight steps over the rack (false for bodyweight moves).
    pub loaded: bool,
    pub sets: Vec<WorkoutSet>,
}

/// The workout in progress.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct Workout {
    pub day: Day,
    /// The day the rotation suggests.
    pub suggested: Day,
    pub exercises: Vec<WorkoutExercise>,
    /// Whether a set is done, so the workout can be finished.
    pub can_finish: bool,
}

/// A set of the workout in progress, by position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, JsObject)]
pub struct SetPosition {
    /// The exercise's index in the workout.
    pub exercise: u32,
    /// The set's index in the exercise.
    pub set: u32,
}

impl From<SetPosition> for core::SetRef {
    fn from(position: SetPosition) -> Self {
        core::SetRef {
            exercise: index(position.exercise),
            set: index(position.set),
        }
    }
}

/// A JS index as a Rust one.
pub fn index(value: u32) -> usize {
    usize::try_from(value).unwrap_or(usize::MAX)
}

/// A muscle's sets this week.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct MuscleWeek {
    pub muscle: Muscle,
    pub name: String,
    pub sets: u32,
    /// The sets a week worth aiming for.
    pub target: u32,
}

/// This week, Monday to Sunday.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct WeekView {
    /// `Week of Mon, Nov 16`.
    pub title: String,
    pub workouts: u32,
    pub goal: u32,
    pub muscles: Vec<MuscleWeek>,
}

/// An exercise of a finished workout.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct HistoryLine {
    pub name: String,
    /// Its sets, such as `25 lb x 10, 9, 9`.
    pub sets: String,
}

/// A finished workout.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct HistoryEntry {
    /// Unique among workouts: its key in lists.
    pub key: u32,
    /// `Wed, Nov 18`.
    pub date: String,
    pub day: Day,
    pub lines: Vec<HistoryLine>,
}

/// An exercise with at least one finished workout.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct LoggedExercise {
    pub id: String,
    pub name: String,
}

/// One workout's sets of an exercise.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct ProgressRow {
    pub date: String,
    pub sets: String,
    /// The heaviest set: `25 lb x 10`.
    pub best: String,
}

/// An exercise's history.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct ProgressView {
    pub id: String,
    pub name: String,
    /// The best set ever, if any.
    pub best: Option<String>,
    /// Its workouts, newest first.
    pub rows: Vec<ProgressRow>,
}

/// An exercise of a day the plan can include.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct PlanRow {
    pub id: String,
    pub name: String,
    pub muscle: Muscle,
    /// `8-12 reps`.
    pub reps: String,
    pub planned: bool,
}

/// A muscle a user's exercise can train.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct MuscleChoice {
    pub muscle: Muscle,
    pub name: String,
}

pub fn muscles(day: core::Day) -> Vec<MuscleChoice> {
    let muscles = core::Muscle::ALL
        .into_iter()
        .filter(|muscle| muscle.day() == day);
    muscles
        .map(|muscle| MuscleChoice {
            muscle: muscle.into(),
            name: muscle.name().to_owned(),
        })
        .collect()
}

/// A dumbbell weight owned.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct RackWeight {
    /// The number as `addWeight` and `removeWeight` take it: `12.5`.
    pub value: String,
    /// With its unit: `12.5 lb`.
    pub label: String,
}

/// The settings.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct SettingsView {
    pub unit: Unit,
    pub weekly_goal: u32,
    pub weights: Vec<RackWeight>,
}

pub fn workout(tracker: &mut core::Tracker) -> Workout {
    let suggested = tracker.suggested_day().into();
    let draft = tracker.workout().clone();
    let exercises = draft
        .exercises
        .iter()
        .filter_map(|exercise| workout_exercise(tracker, exercise))
        .collect();
    Workout {
        day: draft.day.into(),
        suggested,
        exercises,
        can_finish: draft.any_done(),
    }
}

fn workout_exercise(
    tracker: &core::Tracker,
    drafted: &core::DraftExercise,
) -> Option<WorkoutExercise> {
    let exercise = tracker.exercise(&drafted.exercise)?;
    let hint = tracker
        .target(&exercise.id)
        .ok()?
        .hint(exercise, tracker.unit());
    let sets = drafted.sets.iter().map(|set| WorkoutSet {
        weight: set.weight.set_label(tracker.unit()),
        reps: set.reps,
        done: set.done,
    });
    Some(WorkoutExercise {
        id: exercise.id.to_string(),
        name: exercise.name.clone(),
        muscle: exercise.muscle.into(),
        hint,
        loaded: exercise.load == core::Load::Dumbbells,
        sets: sets.collect(),
    })
}

pub fn week(tracker: &core::Tracker, today: core::Date) -> WeekView {
    let week = tracker.week(today);
    let muscles = week.muscles.iter().map(|sets| MuscleWeek {
        muscle: sets.muscle.into(),
        name: sets.muscle.name().to_owned(),
        sets: sets.sets,
        target: core::WEEKLY_SETS_TARGET,
    });
    WeekView {
        title: format!("Week of {}", week.start.label()),
        workouts: week.workouts,
        goal: week.goal,
        muscles: muscles.collect(),
    }
}

pub fn history(tracker: &core::Tracker) -> Vec<HistoryEntry> {
    let entries = tracker.sessions().iter().enumerate().rev();
    entries
        .map(|(key, session)| HistoryEntry {
            key: u32::try_from(key).unwrap_or(u32::MAX),
            date: session.date.label(),
            day: session.day.into(),
            lines: session
                .exercises
                .iter()
                .map(|log| HistoryLine {
                    name: exercise_name(tracker, &log.exercise),
                    sets: core::describe(&log.sets, tracker.unit()),
                })
                .collect(),
        })
        .collect()
}

pub fn exercise_name(tracker: &core::Tracker, id: &core::ExerciseId) -> String {
    tracker
        .exercise(id)
        .map_or_else(|| id.to_string(), |exercise| exercise.name.clone())
}

pub fn logged(tracker: &core::Tracker) -> Vec<LoggedExercise> {
    let ids = tracker.logged_exercises();
    ids.into_iter()
        .map(|id| LoggedExercise {
            id: id.to_string(),
            name: exercise_name(tracker, id),
        })
        .collect()
}

pub fn progress(tracker: &core::Tracker, id: &core::ExerciseId) -> Result<ProgressView, String> {
    let progress = tracker.progress(id).map_err(|error| error.to_string())?;
    let unit = tracker.unit();
    let rows = progress.points.iter().map(|point| ProgressRow {
        date: point.date.label(),
        sets: core::describe(&point.sets, unit),
        best: core::describe(&[point.best], unit),
    });
    Ok(ProgressView {
        id: id.to_string(),
        name: exercise_name(tracker, id),
        best: progress.best.map(|best| core::describe(&[best], unit)),
        rows: rows.collect(),
    })
}

pub fn plan(tracker: &core::Tracker, day: core::Day) -> Vec<PlanRow> {
    let exercises = tracker.exercises().filter(|exercise| exercise.day == day);
    exercises
        .map(|exercise| PlanRow {
            id: exercise.id.to_string(),
            name: exercise.name.clone(),
            muscle: exercise.muscle.into(),
            reps: format!("{} reps", exercise.reps),
            planned: tracker.is_planned(&exercise.id),
        })
        .collect()
}

pub fn settings(tracker: &core::Tracker) -> SettingsView {
    let weights = tracker.rack().weights().map(|weight| RackWeight {
        value: weight.to_string(),
        label: weight.label(tracker.unit()),
    });
    SettingsView {
        unit: tracker.unit().into(),
        weekly_goal: tracker.weekly_goal(),
        weights: weights.collect(),
    }
}
