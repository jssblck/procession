<div align="center">

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B32924%2Fgithub.com%2Fjssblck%2Fprocession.svg?type=small)](https://app.fossa.com/projects/custom%2B32924%2Fgithub.com%2Fjssblck%2Fprocession?ref=badge_small)

</div>

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

# Building and running

This project uses [`cargo`](https://doc.rust-lang.org/cargo/) for build management.

## Building

Running `cargo build` in the root builds all crates in the workspace, optionally with `--release`.

To list available binaries, execute `cargo build --bin` from the root directory:
```
; cargo build --bin
error: "--bin" takes one argument.
Available binaries:
    bench
    it
    procession
```

Then build the binary you're after directly:
```
; cargo build --bin bench
```

## Running

By default, `cargo run` starts and builds `procession`.

To list available binaries, execute `cargo run --bin` from the root directory:
```
; cargo run --bin
error: "--bin" takes one argument.
Available binaries:
    bench
    it
    procession
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

# Licensing

[View third party notices](https://app.fossa.com/reports/9bb31527-cd6e-4186-adbe-f561d236ef2c)

## APSL

This program links against Apple's `Security.Framework` on macOS systems.<br>
The `Security.Framework` source is available here: https://opensource.apple.com/source/Security/<br>
And documentation is available here: https://developer.apple.com/documentation/security
