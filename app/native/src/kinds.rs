//! The core's enums as JS string unions.

use ironless_core as core;
use nsh_core::prelude::*;

/// A day of the push / pull / legs rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PropEnum)]
pub enum Day {
    /// Chest, shoulders and triceps.
    Push,
    /// Back, rear delts and biceps.
    Pull,
    /// Quads, hamstrings, glutes and calves.
    Legs,
}

impl From<core::Day> for Day {
    fn from(day: core::Day) -> Self {
        match day {
            core::Day::Push => Day::Push,
            core::Day::Pull => Day::Pull,
            core::Day::Legs => Day::Legs,
        }
    }
}

impl From<Day> for core::Day {
    fn from(day: Day) -> Self {
        match day {
            Day::Push => core::Day::Push,
            Day::Pull => core::Day::Pull,
            Day::Legs => core::Day::Legs,
        }
    }
}

/// A muscle group whose weekly sets are counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PropEnum)]
pub enum Muscle {
    Chest,
    Shoulders,
    Triceps,
    Back,
    Biceps,
    Quads,
    Hamstrings,
    Glutes,
    Calves,
}

impl From<core::Muscle> for Muscle {
    fn from(muscle: core::Muscle) -> Self {
        match muscle {
            core::Muscle::Chest => Muscle::Chest,
            core::Muscle::Shoulders => Muscle::Shoulders,
            core::Muscle::Triceps => Muscle::Triceps,
            core::Muscle::Back => Muscle::Back,
            core::Muscle::Biceps => Muscle::Biceps,
            core::Muscle::Quads => Muscle::Quads,
            core::Muscle::Hamstrings => Muscle::Hamstrings,
            core::Muscle::Glutes => Muscle::Glutes,
            core::Muscle::Calves => Muscle::Calves,
        }
    }
}

impl From<Muscle> for core::Muscle {
    fn from(muscle: Muscle) -> Self {
        match muscle {
            Muscle::Chest => core::Muscle::Chest,
            Muscle::Shoulders => core::Muscle::Shoulders,
            Muscle::Triceps => core::Muscle::Triceps,
            Muscle::Back => core::Muscle::Back,
            Muscle::Biceps => core::Muscle::Biceps,
            Muscle::Quads => core::Muscle::Quads,
            Muscle::Hamstrings => core::Muscle::Hamstrings,
            Muscle::Glutes => core::Muscle::Glutes,
            Muscle::Calves => core::Muscle::Calves,
        }
    }
}

/// The unit weights are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PropEnum)]
pub enum Unit {
    Lb,
    Kg,
}

impl From<core::Unit> for Unit {
    fn from(unit: core::Unit) -> Self {
        match unit {
            core::Unit::Lb => Unit::Lb,
            core::Unit::Kg => Unit::Kg,
        }
    }
}

impl From<Unit> for core::Unit {
    fn from(unit: Unit) -> Self {
        match unit {
            Unit::Lb => core::Unit::Lb,
            Unit::Kg => core::Unit::Kg,
        }
    }
}
