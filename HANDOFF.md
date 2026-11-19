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

Nothing else exists: no Cargo workspace, no app, no CI, no `AGENTS.md`.

## Product scope (agreed so far)

- Five training days a week: biceps, triceps, legs, back, chest. Shoulders or core may be added or swapped later.
- Dumbbell-only, floor and standing exercises.
- Log per session: exercise, sets, reps, weight, optional note or effort.
- Views: today's workout, history, per-exercise progression with a "beat last time" hint.
- Storage: browser local storage only, with JSON export and import.
- Progression rules and the weekly plan live in a pure Rust core crate; the UI is a thin layer.

## Open questions (ask the user before building the data model)

1. Weekly split: fixed weekdays, or suggest the next day in rotation and allow skips?
2. Exercise library: seed it with floor and standing dumbbell moves per muscle group, or does the user supply the list?
3. Weight handling: a fixed set of owned dumbbell weights, or any number?

## Next steps

1. Get answers to the open questions above.
2. Scaffold the workspace to mirror TiNo (see "Reference: TiNo layout").
3. Write `AGENTS.md` and expand `README.md` with setup and commands.
4. Add `.github/workflows/ci.yml` and enable Pages in the repo settings (Settings > Pages > Source: GitHub Actions,
   or `gh api -X POST repos/casonadams/ironless/pages -f build_type=workflow`).
5. Build the core crate test-first, then the app.

## Reference: TiNo layout (`../TiNo`)

Read these files there before scaffolding; they are the template.

| Path | Role |
|---|---|
| `Cargo.toml` | Workspace: `resolver = "3"`, edition 2024, `rust-version = "1.97"`, `nsh-core = "=0.5.1"` pinned, `[workspace.lints]` |
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

## Conventions to follow (user's global defaults)

- Conventional Commits, imperative subject; add a `Refs: <TICKET>` trailer only if the branch name has a ticket.
- Test-first for deterministic domain behavior; table-driven tests; test observable behavior through public
  interfaces.
- Smallest coherent change; no dead code, placeholders or unrequested TODOs; default to no comments.
- Plain ASCII in repo artifacts.
- No secrets in code, logs or commits. Ask before destructive commands.

## Verification status

No code exists, so there is nothing to build or test. Pages is not enabled yet. The TiNo facts above were read from
its `ci.yml`, `Cargo.toml`, `AGENTS.md` and `README.md`; the current NSH version was not re-checked against upstream,
so confirm the latest compatible `nsh` release before pinning.
