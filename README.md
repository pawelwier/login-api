# run app
cargo watch -q -c -w src/ -x run

# run tests
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"