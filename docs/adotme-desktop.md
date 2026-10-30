# Adotme desktop suite

Display names: PhotoStudio, VectorStudio and LightStudio. Internal Rust package
names, native file formats, existing preferences and repository names remain
stable so existing documents and desktop builds keep working.

All icon variants are deterministic recolors of the same user-supplied mark.
The same crop, alpha mask, white letter and padding are used for every variant.
Home screens use a cached texture rather than decoding an image every frame.
Original licenses, copyright notices and contributor records are retained.
Promotional links now point to Adotme or the fork's repository, and Discord
buttons are removed. Manual update checks use the fork's releases; automatic
startup checks are disabled to avoid upstream replacements and startup work.

PhotoStudio: the Quick Selection brush has New, Add and Subtract buttons in the
options bar. Its choice is stored separately from other selection tools; Alt
temporarily subtracts and Shift adds. Reset Tool resets the brush's mode.

## Validation

PNG/ICO/ICNS decoding and size checks, exact cross-app alpha/white-letter
comparison, Rust formatting/parser checks, and source link audits are run
locally. All three desktop apps and their test targets pass `cargo check --locked --tests`
on Linux. Windows runtime validation and the full repository CI suites remain pending.
Desktop performance has not been benchmarked on the user's Windows machine;
no measured speedup is claimed. Browser deployment is a separate task.

## Windows rebuild

Close the running app, update the local source files, then run
`cargo run --release --locked -p <existing-package-name>`. Recreate a shortcut
if Windows continues to show its cached old icon.
