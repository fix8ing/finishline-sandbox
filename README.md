# finishline-sandbox

A throwaway Rust repository for end-to-end testing the **finishline** CI-autofix
GitHub App. You review a PR and **arm GitHub's native auto-merge** (`gh pr merge
--auto`); that is finishline's cue. On armed PRs it reacts to failing checks:

- **lint failure** (`fmt` / `clippy`) → it pushes a `cargo fmt` / `cargo clippy
  --fix`-style commit.
- **test-compile failure** → a constrained agent pushes a minimal fix.
- **all checks green** → the bot does nothing; GitHub's auto-merge merges the PR.

The bot never merges — it only clears CI. This repo exists to stage each of those
failure classes on demand.

## Layout

| Path | Role |
| --- | --- |
| `src/lib.rs` | `Config` struct + trivial logic + inline unit tests. |
| `src/main.rs` | Thin binary; builds `Config` via `Config::new` (the constructor). |
| `tests/integration.rs` | Constructs `Config` with an explicit **struct literal** — the tier-2 trap. |
| `.github/workflows/ci.yml` | Three separate jobs: `fmt`, `clippy`, `test`. |
| `.finishline.yml` | The bot's policy for this repo. |

Why a library plus a binary? Integration tests in `tests/` link against the
crate's **library** target, not its binary. `Config` therefore lives in
`src/lib.rs` so both `main.rs` and `integration.rs` can reference it. Because
`main.rs` uses the `Config::new` constructor while `integration.rs` uses a
struct literal, adding a field to `Config` (and its constructor) keeps the
library and binary compiling but breaks **only** the integration-test
compilation — the classic tier-2 scenario.

## One-time GitHub setup

After creating the remote and pushing this repo:

1. **Enable auto-merge.** Repo **Settings → General → Pull Requests →** check
   *Allow auto-merge*.
2. **Add a branch protection rule on `main`.** Repo **Settings → Branches → Add
   rule**, branch name pattern `main`, enable *Require status checks to pass
   before merging*, and select the three checks: **fmt**, **clippy**, **test**.
   (They appear in the list only after CI has run at least once, so open a
   throwaway PR first if needed.)
3. **Install the finishline GitHub App** on this repository (App page →
   *Install* / *Configure* → select this repo).

## Test scenarios

Each block creates a branch, stages exactly one failure class, and opens a PR
with `gh`. Run them from the repo root on an up-to-date `main`.

### (a) scenario-fmt — breaks formatting only

Only the `fmt` job fails; `clippy` and `test` stay green. Arm auto-merge, and
finishline should push a `cargo fmt` commit, turning all checks green so GitHub
auto-merges.

```bash
git checkout main && git pull
git checkout -b scenario-fmt
perl -i -pe 's/let config = Config::new/let config    =    Config::new/' src/main.rs
git add -A && git commit -m "scenario-fmt: break rustfmt only"
git push -u origin scenario-fmt
gh pr create --title "scenario-fmt: formatting failure" \
  --body "Breaks only the fmt check. finishline should push a cargo fmt fix; GitHub then auto-merges."
gh pr merge --auto --squash   # arm auto-merge — finishline's cue
```

### (b) scenario-testfix — breaks test compilation

Adds a `verbose` field to `Config` and its constructor in `src/lib.rs`, leaving
the struct literal in `tests/integration.rs` untouched. The `fmt` job stays
green; the `test` job fails with `error[E0063]: missing field verbose`.

> Note: `cargo clippy --all-targets` also compiles the test target, so the
> `clippy` job fails on the **same** compile error. Both failures have one root
> cause and one fix (adding the field to `tests/integration.rs`), which is what
> finishline's constrained test-fix agent produces.

```bash
git checkout main && git pull
git checkout -b scenario-testfix
python3 - <<'PY'
p = "src/lib.rs"
s = open(p).read()
s = s.replace(
    "    pub retries: u32,\n}",
    "    pub retries: u32,\n    /// Whether verbose logging is enabled.\n    pub verbose: bool,\n}",
)
s = s.replace(
    "            retries,\n        }",
    "            retries,\n            verbose: false,\n        }",
)
open(p, "w").write(s)
PY
git add -A && git commit -m "scenario-testfix: add Config.verbose, break test compile"
git push -u origin scenario-testfix
gh pr create --title "scenario-testfix: test-compile failure" \
  --body "Adds Config.verbose but not to tests/integration.rs. finishline's test-fix agent should repair the struct literal; GitHub then auto-merges."
gh pr merge --auto --squash   # arm auto-merge — finishline's cue
```

### (c) scenario-clean — trivial green change

A harmless change that keeps every job green, to verify GitHub's auto-merge
completes with no autofix involved.

```bash
git checkout main && git pull
git checkout -b scenario-clean
perl -i -pe 's/"default"/"clean-demo"/' src/main.rs
git add -A && git commit -m "scenario-clean: trivial green change"
git push -u origin scenario-clean
gh pr create --title "scenario-clean: green change" \
  --body "All checks pass. Arm auto-merge and GitHub should merge with no bot action."
gh pr merge --auto --squash   # arm auto-merge — GitHub merges once green
```

## Local verification

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
