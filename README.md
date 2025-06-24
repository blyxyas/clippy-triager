# autocloseissues

This is a small utility I made while working on the Clippy feature freeze, it
serves to scan for uncommented issues with the `C-bug` label in the Clippy
repository, collect their reproductions for human review, verify and bisect them when
possible.

## Usage

```bash
GH_TOKEN__="token" cargo run -r -- --page <page>
```

Get the 100 issues at page `<page>`. Ideally don't repeat page queries to avoid
unnecessary API usage on the GitHub servers.

---

```bash
cargo run -r -- --bisect
```

Bisect the issues one by one and find a bisected version

---

```bash
cargo r -r -- --repro
```

Verify all the issues and report back if some doesn't reproduce, or has a different error rather than a Clippy one

---

```bash
cargo r -r -- --ignore-comment-count
```

Ignore comment count (normally we only care about issues with 0 comments and no assignees)

## Contributing

Contributing is obviously welcomed, but curently the best way to help this
effort is to triage issues and check that they're beginner friendly and
appropiate to fix in the feature freeze.

See https://github.com/rust-lang/rust-clippy/issues/15086

