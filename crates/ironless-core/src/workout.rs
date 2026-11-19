//! The workout in progress: prefilled sets the user adjusts, marks done and finishes.

use crate::catalog::Load;
use crate::date::Date;
use crate::muscle::Day;
use crate::session::{Draft, DraftExercise, DraftSet, Session, SetRef};
use crate::tracker::{Tracker, TrackerError};

/// The most reps a set may log.
const MAX_REPS: u32 = 999;

/// A direction to step a value in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Up,
    Down,
}

impl Tracker {
    /// The workout in progress, starting the suggested day's when there is none.
    pub fn workout(&mut self) -> &Draft {
        let day = self.next_day;
        if self.draft.is_none() {
            self.draft = Some(self.draft_for(day));
        }
        self.draft.as_ref().expect("a workout was just started")
    }

    /// Starts `day`'s workout in place of the one in progress, whose sets are dropped.
    pub fn choose_day(&mut self, day: Day) {
        self.draft = Some(self.draft_for(day));
    }

    /// Steps the set's weight to the next owned dumbbell up or down; bodyweight sets stay as they are.
    pub fn step_weight(&mut self, at: SetRef, step: Step) -> Result<(), TrackerError> {
        let load = self.load_at(at)?;
        let rack = self.rack.clone();
        let set = self.set_mut(at)?;
        let next = match (load, step) {
            (Load::Bodyweight, _) => None,
            (Load::Dumbbells, Step::Up) => rack.above(set.weight),
            (Load::Dumbbells, Step::Down) => rack.below(set.weight),
        };
        set.weight = next.unwrap_or(set.weight);
        Ok(())
    }

    /// Steps the set's reps by one, within 1 and 999.
    pub fn step_reps(&mut self, at: SetRef, step: Step) -> Result<(), TrackerError> {
        let set = self.set_mut(at)?;
        set.reps = match step {
            Step::Up => (set.reps + 1).min(MAX_REPS),
            Step::Down => set.reps.saturating_sub(1).max(1),
        };
        Ok(())
    }

    /// Marks the set done, or not done again.
    pub fn toggle_done(&mut self, at: SetRef) -> Result<(), TrackerError> {
        let set = self.set_mut(at)?;
        set.done = !set.done;
        Ok(())
    }

    /// Adds a set to the exercise at `exercise`, a copy of its last one, not done.
    pub fn add_set(&mut self, exercise: usize) -> Result<(), TrackerError> {
        let sets = &mut self.draft_exercise(exercise)?.sets;
        let copy = sets.last().map(|set| DraftSet {
            done: false,
            ..*set
        });
        sets.extend(copy);
        Ok(())
    }

    /// Removes the last set of the exercise at `exercise`, keeping at least one.
    pub fn remove_set(&mut self, exercise: usize) -> Result<(), TrackerError> {
        let sets = &mut self.draft_exercise(exercise)?.sets;
        if sets.len() > 1 {
            sets.pop();
        }
        Ok(())
    }

    /// Records the workout's done sets as a workout on `date` and moves the rotation past its day.
    pub fn finish(&mut self, date: Date) -> Result<&Session, TrackerError> {
        let draft = self.draft.as_ref().filter(|draft| draft.any_done());
        let session = draft.ok_or(TrackerError::NothingDone)?.finish(date);
        self.next_day = session.day.next();
        self.draft = None;
        self.sessions.push(session);
        Ok(self.sessions.last().expect("a workout was just recorded"))
    }

    fn set_mut(&mut self, at: SetRef) -> Result<&mut DraftSet, TrackerError> {
        let draft = self.draft.as_mut().ok_or(TrackerError::NoSuchSet)?;
        draft.set_mut(at).ok_or(TrackerError::NoSuchSet)
    }

    fn draft_exercise(&mut self, index: usize) -> Result<&mut DraftExercise, TrackerError> {
        let draft = self.draft.as_mut().ok_or(TrackerError::NoSuchSet)?;
        draft
            .exercises
            .get_mut(index)
            .ok_or(TrackerError::NoSuchSet)
    }

    fn load_at(&self, at: SetRef) -> Result<Load, TrackerError> {
        let draft = self.draft.as_ref().ok_or(TrackerError::NoSuchSet)?;
        let exercise = draft
            .exercises
            .get(at.exercise)
            .ok_or(TrackerError::NoSuchSet)?;
        Ok(self.known(&exercise.exercise)?.load)
    }
}
