use ironless_core::{
    Advice, Date, Day, DraftSet, ExerciseId, Muscle, SetLog, SetRef, Step, Tracker, TrackerError,
    Weight,
};

fn date(text: &str) -> Date {
    text.parse().expect("a valid date")
}

fn id(text: &str) -> ExerciseId {
    ExerciseId::new(text)
}

fn at(exercise: usize, set: usize) -> SetRef {
    SetRef { exercise, set }
}

/// Marks every set of the workout in progress done.
fn do_every_set(tracker: &mut Tracker) {
    let sizes: Vec<usize> = tracker
        .workout()
        .exercises
        .iter()
        .map(|e| e.sets.len())
        .collect();
    for (exercise, size) in sizes.into_iter().enumerate() {
        for set in 0..size {
            tracker.toggle_done(at(exercise, set)).expect("a set");
        }
    }
}

fn names(tracker: &Tracker, day: Day) -> Vec<String> {
    tracker
        .plan_for(day)
        .iter()
        .map(|e| e.name.clone())
        .collect()
}

fn set(weight: u32, reps: u32) -> SetLog {
    SetLog {
        weight: Weight::whole(weight),
        reps,
    }
}

#[test]
fn a_new_tracker_suggests_push_with_the_starting_plan() {
    let mut tracker = Tracker::default();
    let suggested = tracker.suggested_day();
    let workout = tracker.workout().clone();
    assert_eq!(
        (suggested, workout.day, workout.exercises.len()),
        (Day::Push, Day::Push, 6)
    );
    let fresh = DraftSet {
        weight: Weight::whole(5),
        reps: 8,
        done: false,
    };
    let first = &workout.exercises[0];
    assert_eq!(
        (&first.exercise, &first.sets),
        (&id("floor-press"), &vec![fresh; 3])
    );
}

#[test]
fn finishing_records_done_sets_and_moves_the_rotation_on() {
    let mut tracker = Tracker::default();
    tracker.workout();
    tracker.step_weight(at(0, 0), Step::Up).expect("a set");
    tracker.step_reps(at(0, 0), Step::Up).expect("a set");
    tracker.toggle_done(at(0, 0)).expect("a set");
    tracker.toggle_done(at(1, 2)).expect("a set");
    let session = tracker
        .finish(date("2026-11-18"))
        .expect("finished")
        .clone();
    let logged: Vec<(&str, &[SetLog])> = session
        .exercises
        .iter()
        .map(|log| (log.exercise.as_str(), log.sets.as_slice()))
        .collect();
    assert_eq!(session.day, Day::Push);
    assert_eq!(
        logged,
        [
            ("floor-press", &[set(10, 9)][..]),
            ("floor-fly", &[set(5, 10)][..])
        ]
    );
    assert_eq!(
        (tracker.suggested_day(), tracker.workout().day),
        (Day::Pull, Day::Pull)
    );
}

#[test]
fn a_workout_needs_a_done_set_to_finish() {
    let mut tracker = Tracker::default();
    assert_eq!(
        tracker.finish(date("2026-11-18")).err(),
        Some(TrackerError::NothingDone)
    );
    tracker.workout();
    assert_eq!(
        tracker.finish(date("2026-11-18")).err(),
        Some(TrackerError::NothingDone)
    );
}

#[test]
fn skipping_to_another_day_continues_the_rotation_from_it() {
    let mut tracker = Tracker::default();
    tracker.choose_day(Day::Legs);
    assert_eq!(tracker.workout().day, Day::Legs);
    do_every_set(&mut tracker);
    tracker.finish(date("2026-11-18")).expect("finished");
    assert_eq!(tracker.suggested_day(), Day::Push);
}

/// Raises each of the first exercise's sets by `reps` and marks it done.
fn lift_first_exercise(tracker: &mut Tracker, reps: u32) {
    for set in 0..3 {
        for _ in 0..reps {
            tracker.step_reps(at(0, set), Step::Up).expect("a set");
        }
        tracker.toggle_done(at(0, set)).expect("a set");
    }
}

fn first_reps(tracker: &mut Tracker) -> Vec<u32> {
    tracker.workout().exercises[0]
        .sets
        .iter()
        .map(|set| set.reps)
        .collect()
}

#[test]
fn the_next_workout_prefills_one_more_rep_then_the_next_dumbbell() {
    let mut tracker = Tracker::default();
    tracker.workout();
    do_every_set(&mut tracker);
    tracker.finish(date("2026-11-17")).expect("finished");
    let advice = tracker.target(&id("floor-press")).expect("known").advice;
    tracker.choose_day(Day::Push);
    assert_eq!(
        (advice, first_reps(&mut tracker)),
        (Advice::AddReps, vec![9, 9, 9])
    );
    lift_first_exercise(&mut tracker, 3);
    tracker.finish(date("2026-11-19")).expect("finished");
    let press = tracker.exercise(&id("floor-press")).expect("known").clone();
    let target = tracker.target(&press.id).expect("known");
    assert_eq!(
        (target.advice, target.hint(&press, tracker.unit())),
        (
            Advice::AddWeight(Weight::whole(10)),
            "Top of the range last time: move up to 10 lb".to_owned()
        )
    );
}

#[test]
fn weights_step_over_the_owned_dumbbells_only() {
    let mut tracker = Tracker::default();
    tracker.remove_weight(Weight::whole(10));
    tracker.workout();
    tracker.step_weight(at(0, 0), Step::Up).expect("a set");
    assert_eq!(
        tracker.workout().exercises[0].sets[0].weight,
        Weight::whole(15)
    );
    tracker.step_weight(at(0, 0), Step::Down).expect("a set");
    tracker.step_weight(at(0, 0), Step::Down).expect("a set");
    assert_eq!(
        tracker.workout().exercises[0].sets[0].weight,
        Weight::whole(5)
    );
    assert_eq!(
        tracker.step_weight(at(9, 0), Step::Up),
        Err(TrackerError::NoSuchSet)
    );
}

#[test]
fn reps_stay_within_one_and_up() {
    let mut tracker = Tracker::default();
    tracker.workout();
    for _ in 0..20 {
        tracker.step_reps(at(0, 0), Step::Down).expect("a set");
    }
    assert_eq!(tracker.workout().exercises[0].sets[0].reps, 1);
}

#[test]
fn sets_are_added_as_copies_and_one_always_stays() {
    let mut tracker = Tracker::default();
    tracker.workout();
    tracker.toggle_done(at(0, 2)).expect("a set");
    tracker.add_set(0).expect("an exercise");
    let sets = tracker.workout().exercises[0].sets.clone();
    assert_eq!(sets.len(), 4);
    assert_eq!(
        (sets[3].weight, sets[3].reps, sets[3].done),
        (sets[2].weight, sets[2].reps, false)
    );
    for _ in 0..6 {
        tracker.remove_set(0).expect("an exercise");
    }
    assert_eq!(tracker.workout().exercises[0].sets.len(), 1);
}

#[test]
fn the_plan_swaps_exercises_and_an_unstarted_workout_follows() {
    let mut tracker = Tracker::default();
    tracker.workout();
    tracker.toggle_planned(&id("floor-fly")).expect("known");
    tracker.toggle_planned(&id("push-up")).expect("known");
    assert!(names(&tracker, Day::Push).contains(&"Push-up".to_owned()));
    assert!(!names(&tracker, Day::Push).contains(&"Floor fly".to_owned()));
    let workout = tracker.workout();
    assert!(
        workout
            .exercises
            .iter()
            .any(|e| e.exercise == id("push-up"))
    );
    assert_eq!(
        tracker.toggle_planned(&id("bench-press")),
        Err(TrackerError::UnknownExercise("bench-press".into()))
    );
}

#[test]
fn a_started_workout_keeps_its_exercises_when_the_plan_changes() {
    let mut tracker = Tracker::default();
    tracker.workout();
    tracker.toggle_done(at(0, 0)).expect("a set");
    tracker.toggle_planned(&id("floor-press")).expect("known");
    assert_eq!(tracker.workout().exercises[0].exercise, id("floor-press"));
}

#[test]
fn custom_exercises_join_their_muscles_day() {
    let mut tracker = Tracker::default();
    let added = tracker
        .add_exercise("  Zottman curl ", Muscle::Biceps)
        .expect("added");
    assert_eq!(
        tracker.exercise(&added).map(|e| (e.name.as_str(), e.day)),
        Some(("Zottman curl", Day::Pull))
    );
    assert!(names(&tracker, Day::Pull).contains(&"Zottman curl".to_owned()));
    let cases = [
        ("", TrackerError::EmptyName),
        ("   ", TrackerError::EmptyName),
        ("zottman CURL", TrackerError::DuplicateName),
        ("Curl", TrackerError::DuplicateName),
        (&"x".repeat(41), TrackerError::NameTooLong),
    ];
    for (name, error) in cases {
        assert_eq!(
            tracker.add_exercise(name, Muscle::Biceps),
            Err(error),
            "{name:?}"
        );
    }
}

#[test]
fn the_week_counts_workouts_and_sets_per_muscle_from_monday() {
    let mut tracker = Tracker::default();
    tracker.set_weekly_goal(4);
    for day in ["2026-11-15", "2026-11-16", "2026-11-18"] {
        tracker.choose_day(Day::Push);
        do_every_set(&mut tracker);
        tracker.finish(date(day)).expect("finished");
    }
    let week = tracker.week(date("2026-11-19"));
    assert_eq!(
        (week.start, week.workouts, week.goal),
        (date("2026-11-16"), 2, 4)
    );
    let sets: Vec<(Muscle, u32)> = week.muscles.iter().map(|m| (m.muscle, m.sets)).collect();
    let pushed = [Muscle::Chest, Muscle::Shoulders, Muscle::Triceps];
    let expected: Vec<(Muscle, u32)> = Muscle::ALL
        .iter()
        .map(|muscle| (*muscle, if pushed.contains(muscle) { 12 } else { 0 }))
        .collect();
    assert_eq!(sets, expected);
}

#[test]
fn the_weekly_goal_stays_between_one_and_seven() {
    let mut tracker = Tracker::default();
    for (goal, kept) in [(0, 1), (3, 3), (9, 7)] {
        tracker.set_weekly_goal(goal);
        assert_eq!(tracker.weekly_goal(), kept);
    }
}

#[test]
fn progress_lists_workouts_newest_first_with_the_best_set() {
    let mut tracker = Tracker::default();
    tracker.workout();
    tracker.step_weight(at(0, 0), Step::Up).expect("a set");
    tracker.toggle_done(at(0, 0)).expect("a set");
    tracker.toggle_done(at(0, 1)).expect("a set");
    tracker.finish(date("2026-11-16")).expect("finished");
    tracker.choose_day(Day::Push);
    tracker.toggle_done(at(0, 0)).expect("a set");
    tracker.finish(date("2026-11-18")).expect("finished");
    let progress = tracker.progress(&id("floor-press")).expect("known");
    let dates: Vec<Date> = progress.points.iter().map(|point| point.date).collect();
    assert_eq!(dates, [date("2026-11-18"), date("2026-11-16")]);
    assert_eq!(
        progress.best.map(|set| (set.weight, set.reps)),
        Some((Weight::whole(10), 9))
    );
    assert_eq!(tracker.logged_exercises(), [&id("floor-press")]);
}

#[test]
fn save_and_load_round_trip_everything_kept() {
    let mut tracker = Tracker::default();
    tracker.set_unit(ironless_core::Unit::Kg);
    tracker.add_weight(Weight::from_tenths(125)).expect("added");
    tracker
        .add_exercise("Zottman curl", Muscle::Biceps)
        .expect("added");
    tracker.workout();
    do_every_set(&mut tracker);
    tracker.finish(date("2026-11-18")).expect("finished");
    tracker.workout();
    tracker.toggle_done(at(1, 1)).expect("a set");
    let loaded = Tracker::load(&tracker.save()).expect("loads");
    assert_eq!(loaded, tracker);
}

#[test]
fn load_refuses_other_versions_garbage_and_unknown_exercises() {
    let saved = Tracker::default().save();
    let cases = [
        saved.replace("\"version\":1", "\"version\":2"),
        "{".to_owned(),
        saved.replace("floor-press", "bench-press"),
    ];
    for text in cases {
        assert!(Tracker::load(&text).is_err(), "{text}");
    }
}
