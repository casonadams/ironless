//! What the finished workouts add up to: this week's goal and sets per muscle, and an exercise's progress.

use crate::catalog::ExerciseId;
use crate::date::Date;
use crate::muscle::Muscle;
use crate::progression::SetLog;
use crate::session::Session;
use crate::tracker::{Tracker, TrackerError};

/// Hard sets per muscle a week to aim for: more sets grow more muscle, with 10 or more the top group studied (Schoenfeld
/// et al. 2017, doi:10.1080/02640414.2016.1210197).
pub const WEEKLY_SETS_TARGET: u32 = 10;

/// The sets a muscle got in a week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MuscleSets {
    pub muscle: Muscle,
    pub sets: u32,
}

/// The week, Monday to Sunday, that holds a day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Week {
    /// Its Monday.
    pub start: Date,
    /// Workouts finished in it.
    pub workouts: u32,
    /// The weekly workout goal.
    pub goal: u32,
    /// Done sets per muscle, every muscle in rotation order.
    pub muscles: Vec<MuscleSets>,
}

/// One workout's sets of an exercise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressPoint {
    pub date: Date,
    pub sets: Vec<SetLog>,
    /// The heaviest set, the most reps among equals.
    pub best: SetLog,
}

/// An exercise's history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    /// Its workouts, newest first.
    pub points: Vec<ProgressPoint>,
    /// The best set ever: the heaviest, the most reps among equals.
    pub best: Option<SetLog>,
}

impl Tracker {
    /// The week that holds `today`.
    pub fn week(&self, today: Date) -> Week {
        let start = today.week_start();
        let end = start.plus_days(7);
        let sessions: Vec<&Session> = self
            .sessions
            .iter()
            .filter(|session| (start..end).contains(&session.date))
            .collect();
        let muscles = Muscle::ALL.map(|muscle| MuscleSets {
            muscle,
            sets: self.sets_for(muscle, &sessions),
        });
        Week {
            start,
            workouts: u32::try_from(sessions.len()).unwrap_or(u32::MAX),
            goal: self.weekly_goal,
            muscles: muscles.to_vec(),
        }
    }

    fn sets_for(&self, muscle: Muscle, sessions: &[&Session]) -> u32 {
        let logs = sessions.iter().flat_map(|session| &session.exercises);
        let sets = logs
            .filter(|log| {
                self.exercise(&log.exercise)
                    .is_some_and(|e| e.muscle == muscle)
            })
            .map(|log| log.sets.len())
            .sum::<usize>();
        u32::try_from(sets).unwrap_or(u32::MAX)
    }

    /// The exercise's workouts and best set.
    pub fn progress(&self, id: &ExerciseId) -> Result<Progress, TrackerError> {
        self.known(id)?;
        let points: Vec<ProgressPoint> = self
            .sessions
            .iter()
            .rev()
            .filter_map(|session| point(session, id))
            .collect();
        let best = points.iter().map(|point| point.best).max_by_key(rank);
        Ok(Progress { points, best })
    }

    /// The exercises with at least one finished workout, in catalog order.
    pub fn logged_exercises(&self) -> Vec<&ExerciseId> {
        self.exercises()
            .map(|exercise| &exercise.id)
            .filter(|id| self.last_sets(id).is_some())
            .collect()
    }
}

fn point(session: &Session, id: &ExerciseId) -> Option<ProgressPoint> {
    let sets = session.sets_of(id)?;
    let best = sets.iter().copied().max_by_key(rank)?;
    Some(ProgressPoint {
        date: session.date,
        sets: sets.to_vec(),
        best,
    })
}

fn rank(set: &SetLog) -> (u32, u32) {
    (set.weight.tenths(), set.reps)
}
