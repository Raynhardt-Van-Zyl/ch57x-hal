# Releasing `ch57x-hal`

## Prerequisites

1. `cargo login` has been run on this machine.
2. Working tree is clean (except intentional version/changelog updates).
3. `Cargo.toml` uses only crates.io dependencies for published crates.

## Release Checklist

1. Update `Cargo.toml` version.
2. Update `CHANGELOG.md`:
   - move relevant entries from `Unreleased` into a new version/date section.
3. Run checks:
   - `cargo check`
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
4. Package verification:
   - `cargo package`
5. Dry run publish:
   - `cargo publish --dry-run`
6. Publish:
   - `cargo publish`
7. Tag and push:
   - `git tag v<version>`
   - `git push origin main --tags`

## Notes

- If example migration is still in progress, keep release quality focused on the library API and document known example limitations in the changelog.
- Use `cargo owner --list ch57x-hal` to verify crate ownership before first stable releases.
