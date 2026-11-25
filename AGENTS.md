# ast-grep owned fork

This is an owned fork of ast-grep. The `upstream` remote points to `ast-grep/ast-grep`. Keep upstream code changes minimal and upstreamable.

Build with `cargo build --locked -p ast-grep --bin ast-grep` and test with `cargo test -p ast-grep`. Releases use the manually dispatched prerelease workflow.

`make lint` allows `unused_must_use` and `clippy::needless_bool`: the fleet nightly reports them in unchanged upstream code that upstream lints on stable, and rewriting that code would only widen the fork. Remove the allowances when upstream fixes those sites or moves to a toolchain that reports them.
