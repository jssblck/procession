# Procession Integration Test

This crate will eventually be a heavy duty integration test suite.

It'll manage:
- A pool of replicated redis servers.
- A set of clients.
- A pool of `procession` servers.

It'll then put jobs in the queue to be processed, randomly killing parts of its pools
(chaos monkey style), testing that jobs are completed as usual.

As `procession` itself matures this crate will also mature;
while the above is the eventual goal it won't look like that for a while.
