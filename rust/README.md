# Running

With **Cargo** installed, you can run `cargo run -- <args>` in this directory. This will build and run an unoptimized version of the program, useful for simple experiments and development.

To run an optimized version, build with `cargo build --release` and execute the file **./target/release/promethee**.

An usage example is:
```bash
> ./target/release/promethee --version van --weight 0.5 linear 1
[
    -0.5,
    -0.375,
    -0.25,
    -0.125,
    0.0,
    0.125,
    0.25,
    0.375,
    0.5,
]
```