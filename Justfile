test:
    cd avr-tester-fixtures \
    && cargo build --release

    cargo test --release --workspace

clean:
    cargo clean

    cd avr-tester-fixtures \
    && cargo clean
