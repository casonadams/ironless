# HANDOFF

State of the project and what the next agent should do. Delete or replace this file once the scaffold lands.

## Project

`ironless` is a dumbbell-only workout tracker for floor and standing exercises (no bench, no rack). It is built
with [Native Shell](https://nsh.dev) (NSH) and deployed to GitHub Pages, the same way as the sibling repo
`../TiNo`.

- Repo: https://github.com/casonadams/ironless (public, branch `main`, MIT)
- Local: `/Users/cadams/src/github.com/casonadams/ironless`
- Pages URL once deployed: https://casonadams.github.io/ironless/

## Done

- Name chosen: `ironless` (chosen from a list including bellwork and floorset).
- Repo created and pushed with `README.md`, `LICENSE` and `.gitignore` (both copied from TiNo).
- `mise.toml` pins the `nsh` CLI at 0.6.0 (latest stable, released 2026-11-18).

Nothing else exists: no Cargo workspace, no app, no CI, no `AGENTS.md`.

## Product scope (agreed)

- Push / pull / legs rotation over five training days a week:
  - Push: chest, shoulders, triceps.
  - Pull: back, rear delts, biceps.
  - Legs: quads, hamstrings, glutes, calves.
- The app suggests the next day in the rotation; skipping a day does not reset anything.
- Dumbbell-only, floor and standing exercises.
- Log per session: exercise, sets, reps, weight, optional note or effort.
- Views: today's workout, history, per-exercise progression with a "beat last time" hint.
- Storage: browser local storage only, with JSON export and import.
- Progression rules and the plan live in a pure Rust core crate; the UI is a thin layer.

## Decisions

1. **Split**: push / pull / legs rotation, suggested next day, skips allowed.
2. **Exercise library**: seeded with floor and standing dumbbell moves, each tagged with the muscles it works; the
   user can add or swap exercises. Seed list:
   - Chest: floor press, floor fly, push-up.
   - Shoulders: standing overhead press, lateral raise, rear-delt fly.
   - Triceps: lying (floor) extension, overhead extension, kickback.
   - Back: bent-over row, single-arm row, floor pullover, Romanian deadlift.
   - Biceps: curl, hammer curl, cross-body curl.
   - Legs: goblet squat, split squat, reverse lunge, Romanian deadlift, floor hip thrust, calf raise.
3. **Weights**: the user records the dumbbells they own; progression steps to the next owned weight.
4. **Progression**: double progression. Each exercise has a rep range; when every set reaches the top of the range,
   suggest the next owned weight and the bottom of the range. Pre-fill each set with last session's numbers.
5. **Volume**: track weekly sets per muscle against a target of about 10.
6. **Motivation**: a weekly session goal instead of a daily streak; show personal bests and progress; no badges,
   points or guilt messages.
7. **Artwork**: our own SVG line drawings, two poses per lift (start and end), one color so `tintColor` can take a
   theme token. NSH 0.6.0 imports `.svg` files as `<Image>` sources (https://nsh.dev/elements/image/).

## Research behind the decisions

- Twice a week per muscle beats once (Schoenfeld 2016, PubMed 27102172); with volume equated, frequency matters
  little (Schoenfeld 2019, PubMed 30558493). More weekly sets help, with better results above about 9 per muscle
  (PMC8884877).
- Loads from about 6 to 30 reps build similar muscle when sets go near failure (Schoenfeld 2017, PubMed 28834797;
  PMC7927075), so limited home dumbbells work if reps rise.
- Autonomous (intrinsic and identified) motivation correlates with longer fitness-app retention (JMIR mHealth 2026,
  PMC12828317).
- Exercise image sources checked and rejected: free-exercise-db (Unlicense, but gym photos), RepDB free tier (credit
  required, not open data), hasaneyldrm/exercises-dataset (images belong to Gym visual, not licensed for reuse).

## Next steps

1. Scaffold the workspace to mirror TiNo (see "Reference: TiNo layout"), with `nsh-core = "=0.6.0"`.
2. Write `AGENTS.md` and expand `README.md` with setup and commands.
3. Add `.github/workflows/ci.yml` and enable Pages in the repo settings (Settings > Pages > Source: GitHub Actions,
   or `gh api -X POST repos/casonadams/ironless/pages -f build_type=workflow`).
4. Build the core crate test-first, then the app, then the SVG artwork.

## Reference: TiNo layout (`../TiNo`)

Read these files there before scaffolding; they are the template.

| Path | Role |
|---|---|
| `Cargo.toml` | Workspace: `resolver = "3"`, edition 2024, `rust-version = "1.97"`, `nsh-core` pinned exactly (TiNo is on `=0.5.1`; ironless uses `=0.6.0`), `[workspace.lints]` |
| `crates/calc-core` | Domain crate: pure logic, no `nsh-*` dependency, no I/O. For ironless: `crates/ironless-core` |
| `app/native` | Bridge crate (`tino-native`): converts core types to JS values, exports the `#[class]`. Doc comments on every export are required |
| `app/*.tsx`, `app/nsh.json`, `app/tsconfig.json` | TSX UI, NSH config (icon, theme tokens) |
| `mise.toml` | Pins the `nsh` CLI version; keep in step with `nsh-core` in `Cargo.toml` |
| `clippy.toml` | Complexity, argument-count and function-length limits |
| `rust-toolchain.toml` | Adds the `wasm32-unknown-unknown` target |
| `.github/workflows/ci.yml` | Jobs `rust`, `app`, `web`, `deploy` |
| `AGENTS.md` | Layering rules, commands, conventions; model for ironless's own |

Key facts from TiNo:

- Layers point down only: Domain (core crate) -> Bridge (`app/native`) -> App (TSX). The bridge holds no behavior and
  the app decides no domain rules.
- Storage is the app's job. TiNo saves a core snapshot to browser local storage after each change. `nsh_core::storage`
  is not reachable from class methods called by TSX. Saved state carries a version; a change an older snapshot cannot
  be read under means a new version.
- Never edit `app/.nsh/` (generated; gitignored via `.nsh/`).
- Colors come from theme tokens only, no raw colors in components.
- Checks that must pass before commit:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd app && nsh check && nsh test
```

- CI builds the web target with `nsh package --target web --out "$GITHUB_WORKSPACE/dist"`, touches
  `dist/<name>/.nojekyll`, uploads `dist/<name>` with `actions/upload-pages-artifact@v5`, and deploys with
  `actions/deploy-pages@v5` on `main` only. CI env sets `CC_wasm32_unknown_unknown: clang`,
  `AR_wasm32_unknown_unknown: llvm-ar`, and `CARGO_TARGET_DIR` to the workspace `target`; the `web` job installs
  `clang llvm`. Local web builds on macOS use Homebrew's `/opt/homebrew/opt/llvm/bin/{clang,llvm-ar}`.
- The `<name>` in `dist/<name>` is the app name from `app/nsh.json`; check it so the Pages path matches the repo name.
- `../smartalex` is a second, TSX-only NSH app (no Rust crate) if a lighter reference is useful.

## NSH 0.6.0 notes

- mise hides 0.6.0 behind a minimum release age; `mise install` works because `mise.toml` names the version.
- `nsh init --template mixed` (0.6.0) generates `nsh.json` with `"rust"`, a crate using `nsh-core` and `nsh-base`
  0.6.0, `#[native_module]`, `#[class]` and `#[module]`, and an `App.test.tsx` using `nsh/test`. A generated app
  passed `nsh check` and `nsh test` locally.
- Breaking: an app that declares `capabilities` in `nsh.json` must also list `application` and `homeDir` if it uses
  them. Release notes: https://github.com/longbridge/nsh/releases/tag/v0.6.0

## Conventions to follow (user's global defaults)

- Conventional Commits, imperative subject; add a `Refs: <TICKET>` trailer only if the branch name has a ticket.
- Test-first for deterministic domain behavior; table-driven tests; test observable behavior through public
  interfaces.
- Smallest coherent change; no dead code, placeholders or unrequested TODOs; default to no comments.
- Plain ASCII in repo artifacts.
- No secrets in code, logs or commits. Ask before destructive commands.

## Verification status

No code exists yet, so there is nothing to build or test. Pages is not enabled yet. NSH 0.6.0 was confirmed as the
latest stable release on 2026-11-18 and checked with a throwaway `nsh init --template mixed` app (`nsh check`,
`nsh test` passed).
