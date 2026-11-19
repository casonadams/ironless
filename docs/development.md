# Developing ironless

## Tools

- [mise](https://mise.jdx.dev), which pins the `nsh` CLI in `mise.toml`.
- Rust stable 1.97 or newer with the `wasm32-unknown-unknown` target (`rust-toolchain.toml` adds it).
- For web builds, a clang that targets WebAssembly plus `llvm-ar` (on macOS, Homebrew's `llvm`).

## Commands

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
app/                    the TSX app: App.tsx, store.ts, screens/, components/, art/ (exercise drawings),
                        why.ts (the Why view's text)
docs/                   why.md (the research behind the rules) and this file
```

[AGENTS.md](../AGENTS.md) describes the architecture and conventions in more detail.
