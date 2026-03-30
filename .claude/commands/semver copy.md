# semver

Analyze git history since the last release tag and propose a semantic version bump for this Rust project.

## Usage
```
claude semver [--dry-run] [--from <tag>] [--bump <major|minor|patch>]
```

## Instructions

### Phase 1 — Gather context (silent, no output yet)

Run all of the following before presenting anything to the user:

1. Read `Cargo.toml` — extract current version and crate name. If this is a workspace, collect all member crate versions.
2. Run `git tag --sort=-version:refname | head -1` to find the last version tag. If no tag exists, treat the entire commit history as the range and note this is a first release.
3. Run `git log <last_tag>..HEAD --pretty=format:"%H %s"` to collect all commits since the last release.
4. Run `git diff <last_tag>..HEAD -- src/` for the full source diff.
5. Check whether `cargo semver-checks` is available in PATH.

If `--from <tag>` is provided, use that tag instead of the auto-detected last tag.

### Phase 2 — cargo semver-checks

`cargo semver-checks` is required. Do not proceed without it.

- If it is installed, run it and capture the output.
- If it is not installed, pause and prompt the user:

```
cargo semver-checks is required for reliable API surface analysis.
Install it now? (cargo install cargo-semver-checks) [y/n]
```

If the user declines, abort with a clear message. Do not fall back to diff heuristics.

### Phase 3 — Classify the bump

**Primary signal — commit messages** (conventional commits):

| Pattern | Bump |
|---|---|
| `BREAKING CHANGE:` in body, or `!` after type | major |
| `feat:` | minor |
| `fix:`, `perf:`, `refactor:` | patch |
| `chore:`, `docs:`, `test:`, `ci:` | none |

**Safety net — cargo semver-checks output:**

If `cargo semver-checks` reports any breaking changes, the bump must be **major** regardless of commit messages. Flag the discrepancy clearly to the user.

**Override:** If `--bump <level>` is provided, skip classification and use the specified level. Still show the analysis summary so the user can verify.

### Phase 4 — Present proposal

Show the full proposal before touching any files:

```
Current version : 1.3.2
Proposed bump   : MINOR → 1.4.0

Reason:
  + feat: add parse_config() to Config builder  (commit abc1234)
  + feat: derive Clone on OutputFormat           (commit def5678)

cargo semver-checks: no breaking changes detected ✓

Files that will be modified:
  • Cargo.toml        (version: "1.3.2" → "1.4.0")
  • CHANGELOG.md      (prepend new section)
  • commit.md         (suggested commit message)

Git actions (local only, no push):
  • git tag v1.4.0

To release after confirming:
  git commit -F commit.md
  git push && git push --tags
  cargo publish

Proceed? [y] confirm  [n] cancel  [M] force major  [m] force minor  [p] force patch
```

Wait for a single response before proceeding. Do not ask follow-up questions.

If `--dry-run` is set, print the proposal and stop here. Do not modify any files.

### Phase 5 — Execute

Perform in this exact order:

1. Write new version string to `Cargo.toml`. For workspaces, update all affected member crates.
2. Prepend a new section to `CHANGELOG.md` in [Keep a Changelog](https://keepachangelog.com) format. Create the file if it does not exist.
3. Write the suggested commit message to `commit.md` at the repo root. Use conventional commit format: `chore: release v<version>` as the subject, with a brief summary of changes in the body.
4. Stage with `git add Cargo.toml CHANGELOG.md commit.md`.
5. Create local tag: `git tag v<new_version>`.

**Do not** run `git commit`, `git push`, or `cargo publish`.

### Phase 6 — Summary

```
✓ Cargo.toml updated to 1.4.0
✓ CHANGELOG.md updated
✓ commit.md written with suggested commit message
✓ Files staged (not committed)
✓ Tag v1.4.0 created locally

When ready to release:
  git commit -F commit.md
  git push && git push --tags
  cargo publish
```

## Edge cases

| Situation | Behavior |
|---|---|
| No previous tag | Prompt: is this a first release? Suggest `0.1.0` or `1.0.0` |
| Dirty working tree | Warn, block tagging. Allow `--dry-run` to proceed |
| Tag already exists locally | Abort with message asking user to delete it manually |
| Workspace with multiple crates | Analyze each crate independently; only bump affected ones |
| Version in Cargo.toml already ahead of suggestion | Warn: "Cargo.toml is already at X, which is ahead of suggested Y. Continuing will set version to Z." |
