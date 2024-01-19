# regchest_command

Zingolib command library for building scenarios.

dependencies: zcashd, zcash-cli, lightwalletd.
- Symlink zcashd, zcash-cli and lightwalletd binaries to regchest/bin/ before use.

Examples in tests/usage_examples.rs.

client-server usage example in src/regchest_test_server.rs and src/regchest_test_client.rs.
- run "cargo run --bin regchest_test_server" to start TCP Listener then in another terminal run "cargo run --bin regchest_test_client".
