//! The workout tracker as `App.tsx` imports it from `ironless/native`. Behavior lives in `ironless-core`; this crate
//! only converts between it and JS values.

mod kinds;
mod views;

use ironless_core as core;
pub use kinds::{Day, Muscle, Unit};
use nsh_core::prelude::*;
pub use views::{
    HistoryEntry, HistoryLine, LoggedExercise, MuscleChoice, MuscleWeek, PlanRow, ProgressRow,
    ProgressView, RackWeight, SetPosition, SettingsView, WeekView, Workout, WorkoutExercise,
    WorkoutSet,
};

#[native_module("ironless/native")]
fn native(registry: &mut Registry) {
    registry.class::<Tracker>();
}

/// A workout tracker whose state lives in Rust.
#[class]
#[derive(Default)]
pub struct Tracker {
    core: core::Tracker,
}

fn message(error: impl std::fmt::Display) -> String {
    error.to_string()
}

fn step(up: bool) -> core::Step {
    if up { core::Step::Up } else { core::Step::Down }
}

#[class]
impl Tracker {
    /// A tracker with the starting plan, a 5 to 50 lb rack and no workouts.
    pub fn new() -> Self {
        Self::default()
    }

    /// Everything the tracker keeps, as text `restore` reads: the saved state and the backup file.
    pub fn save(&self) -> String {
        self.core.save()
    }

    /// Becomes the tracker `save` wrote. Throws, leaving this tracker as it was, for text that is not a backup
    /// this version reads.
    pub fn restore(&mut self, text: String) -> Result<(), String> {
        self.core = core::Tracker::load(&text).map_err(message)?;
        Ok(())
    }

    /// The workout in progress, starting the suggested day's when there is none.
    pub fn workout(&mut self) -> Workout {
        views::workout(&mut self.core)
    }

    /// Starts `day`'s workout in place of the one in progress, whose sets are dropped.
    pub fn choose_day(&mut self, day: Day) {
        self.core.choose_day(day.into());
    }

    /// Steps the set's weight to the next owned dumbbell up (or down). Throws for a set the workout lacks.
    pub fn step_weight(&mut self, at: SetPosition, up: bool) -> Result<(), String> {
        self.core.step_weight(at.into(), step(up)).map_err(message)
    }

    /// Steps the set's reps by one up (or down), within 1 and 999. Throws for a set the workout lacks.
    pub fn step_reps(&mut self, at: SetPosition, up: bool) -> Result<(), String> {
        self.core.step_reps(at.into(), step(up)).map_err(message)
    }

    /// Marks the set done, or not done again. Throws for a set the workout lacks.
    pub fn toggle_done(&mut self, at: SetPosition) -> Result<(), String> {
        self.core.toggle_done(at.into()).map_err(message)
    }

    /// Adds a set to the exercise at `exercise`, a copy of its last one.
    pub fn add_set(&mut self, exercise: u32) -> Result<(), String> {
        self.core.add_set(views::index(exercise)).map_err(message)
    }

    /// Removes the last set of the exercise at `exercise`, keeping one.
    pub fn remove_set(&mut self, exercise: u32) -> Result<(), String> {
        self.core
            .remove_set(views::index(exercise))
            .map_err(message)
    }

    /// Records the workout's done sets on `date` (`YYYY-MM-DD`) and moves the rotation past its day. Throws when no
    /// set is done or the date is not one.
    pub fn finish(&mut self, date: String) -> Result<(), String> {
        let date: core::Date = date.parse().map_err(message)?;
        self.core.finish(date).map(|_| ()).map_err(message)
    }

    /// The week, Monday to Sunday, that holds `today` (`YYYY-MM-DD`). Throws for text that is not a date.
    pub fn week(&self, today: String) -> Result<WeekView, String> {
        let today: core::Date = today.parse().map_err(message)?;
        Ok(views::week(&self.core, today))
    }

    /// The finished workouts, newest first.
    pub fn history(&self) -> Vec<HistoryEntry> {
        views::history(&self.core)
    }

    /// The exercises with at least one finished workout.
    pub fn logged(&self) -> Vec<LoggedExercise> {
        views::logged(&self.core)
    }

    /// The exercise's workouts and best set. Throws for an id no exercise has.
    pub fn progress(&self, id: String) -> Result<ProgressView, String> {
        views::progress(&self.core, &core::ExerciseId::new(id))
    }

    /// Every exercise of `day`, and whether workouts include it.
    pub fn plan(&self, day: Day) -> Vec<PlanRow> {
        views::plan(&self.core, day.into())
    }

    /// The muscles `day` trains, which an exercise of the user's own for that day can name.
    pub fn muscles(&self, day: Day) -> Vec<MuscleChoice> {
        views::muscles(day.into())
    }

    /// Includes the exercise in its day's workouts, or leaves it out. Throws for an id no exercise has.
    pub fn toggle_planned(&mut self, id: String) -> Result<(), String> {
        self.core
            .toggle_planned(&core::ExerciseId::new(id))
            .map_err(message)
    }

    /// Adds a dumbbell exercise of the user's own for `muscle`, included in that muscle's day. Throws for an empty,
    /// too long or taken name.
    pub fn add_exercise(&mut self, name: String, muscle: Muscle) -> Result<(), String> {
        self.core
            .add_exercise(&name, muscle.into())
            .map(|_| ())
            .map_err(message)
    }

    /// The unit, the weekly goal and the dumbbells owned.
    pub fn settings(&self) -> SettingsView {
        views::settings(&self.core)
    }

    /// Writes every weight in `unit`; the numbers stay as they are.
    pub fn set_unit(&mut self, unit: Unit) {
        self.core.set_unit(unit.into());
    }

    /// Sets the workouts a week to aim for, kept within 1 to 7.
    pub fn set_weekly_goal(&mut self, goal: u32) {
        self.core.set_weekly_goal(goal);
    }

    /// Adds a dumbbell weight (`12.5`) to the rack. Throws for text that is not a weight above 0.
    pub fn add_weight(&mut self, weight: String) -> Result<(), String> {
        let weight: core::Weight = weight.parse().map_err(message)?;
        self.core.add_weight(weight).map_err(message)
    }

    /// Removes a dumbbell weight (`12.5`) from the rack. Throws for text that is not a weight.
    pub fn remove_weight(&mut self, weight: String) -> Result<(), String> {
        let weight: core::Weight = weight.parse().map_err(message)?;
        self.core.remove_weight(weight);
        Ok(())
    }
}
