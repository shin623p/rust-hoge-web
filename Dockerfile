FROM jimmycuadra/rust
EXPOSE 3000
COPY Cargo.toml /source
COPY src/main.rs /source/src/
CMD cargo run
