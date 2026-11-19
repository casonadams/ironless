# ironless

A dumbbell-only workout tracker for floor and standing exercises: no bench, no rack. Live at
**https://casonadams.github.io/ironless/**.

Built with [Native Shell](https://nsh.dev): the training rules are a Rust crate with no UI dependency, and a small
TSX app draws the screens.

## What it does

- **Push / pull / legs rotation**: push is chest, shoulders and triceps; pull is back, rear delts and biceps; legs
  are quads, hamstrings, glutes and calves. The app suggests the next day; pick another and the rotation continues
  from the one you did.
- **Today**: each exercise with a drawing of its start and end positions, a hint from last time, and its sets
  prefilled. Step the weight over the dumbbells you own and the reps up or down, tap a set's number when it is done,
  add or remove sets, then Finish.
- **Double progression**: each exercise has a rep range (8-12 for most). Add a rep per set until every set reaches
  the top, then the next workout moves up to your next dumbbell and back to the bottom of the range. Bodyweight
  moves, and dumbbells already at your heaviest, keep adding reps.
- **History**: finished workouts, newest first.
- **Progress**: this week's sets per muscle against 10, each exercise's workouts and best set, and a **Why** view
  that explains each rule and links to the research behind it.
- **Plan**: include or leave out each day's exercises (push-ups, kickbacks, cross-body curls, reverse lunges and
  single-leg Romanian deadlifts are ready to swap in), or add your own.
- **Settings**: a weekly workout goal (5 by default), lb or kg, the dumbbells you own (5 to 50 lb to start), and
  backups: download one as a file and restore it on another device.
- **Kept in this browser** only, in its local storage.

## Why it works this way

In short: train each muscle about twice a week, count hard sets per muscle, take sets close to failure with whatever
weight you have, add reps before weight, and aim for a weekly goal rather than a streak that one missed day breaks.
[docs/why.md](docs/why.md) covers what each study found, how strong the evidence is, and what the app does about it.

## Development

[docs/development.md](docs/development.md) has the tools, commands and layout; [AGENTS.md](AGENTS.md) the
architecture and conventions.

## License

[MIT](LICENSE)
