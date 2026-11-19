# AGENTS.md

Guidance for coding agents and contributors working in this repository.

## Architecture

Three layers; dependencies point down only.

| Layer | Path | Owns | Must not |
|---|---|---|---|
| Domain | `crates/ironless-core` | The push / pull / legs rotation, the exercise catalog and plan, the dumbbell rack, weights and units, double progression and hints, the workout in progress, finished sessions, weekly totals, progress, save/load | Depend on `nsh-*` or do I/O |
| Bridge | `app/native` | Converting core types to JS values (`PropEnum`, `JsObject`, the `#[class] Tracker`) | Hold behavior; it only converts |
| App | `app/*.ts(x)` | Drawing the views, tabs, gestures, storage I/O, backups, the exercise drawings | Decide training rules |

- `Tracker` in the core is the single aggregate. Workout changes live in `workout.rs`, plan and settings in
  `tracker.rs`, weekly totals and progress in `summary.rs`, save/load in `persist.rs`.
- The app reads the tracker through `read()` and changes it through `change()` in `app/store.ts`, which bumps a
  revision signal and saves `Tracker.save()` under `ironless.tracker` after every change. The screens show only
  once `restoreTracker()` has finished, so no change can overwrite the saved tracker. Rust's `nsh_core::storage`
  is not reachable from class methods called by TSX.
- Dates cross the bridge as local `YYYY-MM-DD` text from `today()` in `app/store.ts`; the core never reads a clock.
- Backups: Settings posts `backup.download` to the page host (`app/host.js`), which downloads the save text as a
  file; restoring picks a file with `Picker.pickFiles` and hands its text to `Tracker.restore`.

## Commands

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd app && nsh check && nsh test
```

All five must pass before a commit. CI (`.github/workflows/ci.yml`) runs them, packages the web build and deploys
`main` to GitHub Pages. A local web build needs `CC_wasm32_unknown_unknown` and `AR_wasm32_unknown_unknown` pointing
at a WebAssembly clang and `llvm-ar` (see README).

## Quality gates

- `clippy.toml`: cognitive complexity at most 5, at most 3 arguments (including `self`), functions at most 60 lines.
  Split by responsibility or bundle related arguments into a type (`SetRef`, `SetPosition`). Do not add `#[allow]`.
- No dead code, placeholders or stubs.

## Conventions

- **Training rules** come from the plan agreed for this project: push / pull / legs rotation with skips, about 10
  sets per muscle a week (`WEEKLY_SETS_TARGET`), double progression over the dumbbells the user owns, a weekly
  workout goal instead of a daily streak, no badges or guilt messages.
- **Exercises are floor or standing dumbbell moves**: no bench, no rack. Seeded exercises live in
  `catalog.rs`; their ids are saved data, so never rename one. A new seeded exercise needs a drawing.
- **Weights are tenths of the unit** (`Weight`); the unit only labels them and converts nothing.
- **Drawings** are `app/art/<exercise id>.svg`, registered in `app/art/index.ts`: 240x120, the start pose on the
  left and the end pose on the right of an arrow, drawn in one color (`#000`, strokes 5 wide, round caps, a filled
  head of radius 7, dumbbells as filled circles of radius 6) so `tintColor` can take a theme token. The app test
  "every seeded exercise has a drawing" catches a missing or stray one.
- **Doc comments on everything `app/native` exports** are required: NSH generates `.nsh/types` TypeScript from them
  and warns without them. Keep them short and about behavior.
- **Colors come from theme tokens only** (`bg-card`, `text-muted-foreground`, `token("foreground")`), defined in
  `app/theme.json`. No raw colors or palette classes in components.
- **Saved state** has a version (`SNAPSHOT_VERSION` in `persist.rs`). A new field with a serde default keeps the
  version; any change an older snapshot cannot be read under means a new version. The app sets a save it cannot read
  aside under `ironless.tracker.unreadable`, starts fresh and tells the user; it never overwrites one.
- **NSH** is pinned in `mise.toml`, as `=0.6.0` for `nsh-core` in `Cargo.toml` and as `"nsh"` in `app/nsh.json`;
  upgrade all three together and rerun every check. Never edit `app/.nsh/`; it is generated.
- **The app icon** (`app/icon.png`, set in `nsh.json`; NSH derives the favicon and web app icons from it) is rendered
  from `app/icon.svg`. ImageMagick's own SVG renderer drops strokes, so render with QuickLook and cut the corners:
  `qlmanage -t -s 1024 -o /tmp app/icon.svg && magick /tmp/icon.svg.png \( -size 1024x1024 xc:none -fill white -draw
  "roundrectangle 0,0,1023,1023,224,224" \) -alpha set -compose DstIn -composite -strip app/icon.png`.

## Tests

- Domain rules: unit tests beside the code, table-driven where one rule holds across inputs; whole workflows in
  `crates/ironless-core/tests/tracker.rs` through the public API.
- App: `app/App.test.tsx`, headless through `nsh test`, pressing views by `id:` and asserting on texts, enabled
  state, storage, host posts and picked files. Test behavior a user can see, not wiring.

## Commits

Conventional Commits (`feat(core):`, `fix(app):`, `docs:`), imperative subject, a short bullet body.
