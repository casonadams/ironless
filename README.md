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
- **Progress**: this week's sets per muscle against about 10, and each exercise's workouts and best set.
- **Plan**: include or leave out each day's exercises (push-ups, kickbacks, cross-body curls, reverse lunges and
  single-leg Romanian deadlifts are ready to swap in), or add your own.
- **Settings**: a weekly workout goal (5 by default), lb or kg, the dumbbells you own (5 to 50 lb to start), and
  backups: download one as a file and restore it on another device.
- **Kept in this browser** only, in its local storage.

### Why these rules

- Training a muscle twice a week beats once (Schoenfeld et al. 2016, PubMed 27102172); with equal weekly volume the
  frequency matters little (Schoenfeld et al. 2019, PubMed 30558493). Push / pull / legs over five days trains each
  muscle about 1.7 times a week.
- More weekly sets give more growth, with better results above about 9 per muscle (systematic review, PMC8884877).
- Loads from about 6 to 30 reps build similar muscle when sets go near failure (Schoenfeld et al. 2017, PubMed
  28834797), so a limited set of home dumbbells works if reps rise.
- People who train because they want to stay with fitness apps longer (JMIR mHealth 2026, PMC12828317): the app
  shows progress and offers swaps rather than badges, points or streak guilt.

## Development

Tools: [mise](https://mise.jdx.dev) (pins the `nsh` CLI in `mise.toml`), Rust stable 1.97 or newer with the
`wasm32-unknown-unknown` target (`rust-toolchain.toml` adds it), and for web builds a clang that targets
WebAssembly plus `llvm-ar` (on macOS, Homebrew's `llvm`).

```sh
mise install                                    # the pinned nsh

cargo test --workspace                          # the domain's tests
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all

cd app
nsh check                                       # type-checks the TSX against the Rust exports
nsh test                                        # the app's headless tests
nsh run --target web                            # dev server with hot reload

# a production web build (macOS)
CC_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/clang \
AR_wasm32_unknown_unknown=/opt/homebrew/opt/llvm/bin/llvm-ar \
  nsh package --target web                      # writes app/.nsh/out/ironless/
```

Every push to `main` runs the checks above in GitHub Actions and deploys the web build to GitHub Pages
(`.github/workflows/ci.yml`).

## Layout

```
crates/ironless-core/   the domain: rotation, exercises, rack, progression, workouts, totals, save/load;
                        no UI or NSH dependency
app/native/             the Rust bridge: exports the core to TSX as `ironless/native`
app/                    the TSX app: App.tsx, store.ts, screens/, components/, art/ (exercise drawings)
```

[AGENTS.md](AGENTS.md) describes the architecture and conventions in more detail.

## License

[MIT](LICENSE)
