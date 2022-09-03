# procession

The server binary for `procession`: the fault tolerant, performant, language agnostic background job server.

## UI

The UI is served at the root of the HTTP server.

It is responsible for:

- Rendering queues and jobs in progress.
- Displaying job statistics in a human readable format.
- Providing basic queue and job management for human operators.

## Job management

Due to the call and response nature of job management, all job management is handled on the HTTP server.

## Worker APIs

Procession supports workers with both a TCP and HTTP API.
The intention is that clients wishing to be maximally performant should use the TCP API,
while clients that want a simpler way to integrate or in environments that don't support non-HTTP ports
can fall back to the HTTP API.

For example, it's not uncommon in some corporate environments to have restrictions on the
ports services may use to make outbound communication.
In such an environment it may be helpful to be able to connect to the job server over HTTP instead,
despite the performance overhead this brings.

### HTTP

The HTTP worker API is implemented via submitting new jobs as a standard HTTP POST,
and receiving job assignments via long-polling a HTTP GET.

As with most long polling implementations the server keeps the connection open for a set amount of time,
after which the client receives an empty response and is expected to re-initialize the same request.

The intent with a long polling fallback is to provide the ability for clients to operate in enviroments
that are hostile to plain TCP. Such environments are usually also relatively picky about killing idle HTTP connections,
so Procession's long polling implementation generally trades performance in order to operate more nicely with
proxies that may be configured with very severe timeouts.

For improved performance, switch to the TCP server if possible.

### TCP

The TCP worker API is implemented as a plain TCP connection where the client submits jobs by sending a client initiated payload,
and receives job assignments as server initiated payloads.

The TCP API is used purely for submitting jobs and receiving job assignment.

Any job management or statistics gathering that the client wishes to do is performed via the HTTP API;
the idea is that this simplifies both the client and server implementations by ensuring that when connected
over TCP both ends of the connection are solely concerned with one kind of task.
