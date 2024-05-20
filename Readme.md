# cli_test

This is a test repo for `command_rpc`, a cli/rpc project of mine. In `expanded.rs` one
can find the result of macro expansion applied to the main crate. `get_ast.rs` does not
contain production code but the equivalent of the cli in the `main.rs` file written with
the `clap` framework without overlaying procedural macros.

You can test usage of `command_rpc` by cloning this repository and executing
`cargo expand > expanded.rs | cargo rustc` (powershell).

Have fun!