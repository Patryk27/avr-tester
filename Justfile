test *args:
    cd avr-tester-fixtures \
    && cargo build --release

    cargo test --release --workspace -- {{ args }}

test-ccb *args:
    cd avr-tester-fixtures \
    && cargo build --release --features custom-compiler-builtins

    cargo test --release --workspace -- {{ args }}

clean:
    cargo clean

    cd avr-tester-fixtures \
    && cargo clean
