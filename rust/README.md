Instructions
------------

`cd spell-correct; cargo build --release && ./target/release/spell-correct [word]+`

If you want to build it in debug mode:

`cd spell-correct; cargo build && ./target/debug/spell-correct [word]+`

Note
----

On my machine, the debug binary takes nearly a minute to run, the release binary takes around a second.
