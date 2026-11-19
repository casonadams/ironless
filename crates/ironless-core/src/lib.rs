//! ironless's workout domain: the push / pull / legs rotation, floor and standing dumbbell exercises, the dumbbells
//! owned, double progression, the workout in progress, finished workouts, weekly totals and save/load. No UI or I/O.

pub mod catalog;
pub mod date;
pub mod muscle;
pub mod persist;
pub mod progression;
pub mod session;
pub mod summary;
pub mod tracker;
pub mod weight;
pub mod workout;

pub use catalog::{Exercise, ExerciseId, Load, RepRange, SETS_PER_EXERCISE};
pub use date::{Date, InvalidDate};
pub use muscle::{Day, Muscle};
pub use persist::LoadError;
pub use progression::{Advice, SetLog, Target, describe};
pub use session::{Draft, DraftExercise, DraftSet, ExerciseLog, Session, SetRef};
pub use summary::{MuscleSets, Progress, ProgressPoint, WEEKLY_SETS_TARGET, Week};
pub use tracker::{MAX_NAME_CHARS, Tracker, TrackerError};
pub use weight::{InvalidWeight, Rack, RackError, Unit, Weight};
pub use workout::Step;
