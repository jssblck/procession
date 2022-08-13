# Procession

`procession` is a fault tolerant, performant, language agnostic background job server.

... or at least it will be, one day. So far it's in the early stages.

- [Server](./server/README.md)
- [Clients](./client/README.md)
- [Housekeeping](./housekeeping/README.md)

# High Level Architecture

- A `procession` server manages jobs using a backing [`redis`](https://redis.io/) database.
- One or more `client` instances push jobs to the server to be processed.
- One or more `client` instances consume jobs from the server.

For more detail, see [Architecture](./ARCHITECTURE.md).

# Building

This project uses [`cargo`](https://doc.rust-lang.org/cargo/) for build management.

To list available binaries, execute `cargo run` from the root directory:
```
; cargo r
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: bench, it, procession
```

From there you can run a specific binary:
```
; cargo run --bin procession
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/procession`
2022-08-13T23:28:39.620002Z  INFO procession: 🤔 Checking connection to redis://localhost:6379/0
2022-08-13T23:28:39.623662Z  INFO procession: 💚 Redis is running at redis://localhost:6379/0
2022-08-13T23:28:39.623890Z  INFO procession: 👩🏻‍💻 Starting procession 0.1.0 on 0.0.0.0:3000
2022-08-13T23:28:39.624404Z  INFO procession: ✨ Serving procession 0.1.0 on 0.0.0.0:3000
```
