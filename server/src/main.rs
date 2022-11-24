//! The server binary for `procession`: the fault tolerant, performant, language agnostic background job server.
//!
//! # UI
//!
//! The UI is served at the root of the HTTP server.
//!
//! It is responsible for:
//!
//! - Rendering queues and jobs in progress.
//! - Displaying job statistics in a human readable format.
//! - Providing basic queue and job management for human operators.
//!
//! # Job management
//!
//! Due to the call and response nature of job management, all job management is handled on the HTTP server.
//!
//! # Worker APIs
//!
//! Procession supports workers with both a TCP and HTTP API.
//! The intention is that clients wishing to be maximally performant should use the TCP API,
//! while clients that want a simpler way to integrate or in environments that don't support non-HTTP ports
//! can fall back to the HTTP API.
//!
//! For example, it's not uncommon in some corporate environments to have restrictions on the
//! ports services may use to make outbound communication.
//! In such an environment it may be helpful to be able to connect to the job server over HTTP instead,
//! despite the performance overhead this brings.
//!
//! ## HTTP
//!
//! The HTTP worker API is implemented via submitting new jobs as a standard HTTP POST,
//! and receiving job assignments via long-polling a HTTP GET.
//!
//! As with most long polling implementations the server keeps the connection open for a set amount of time,
//! after which the client receives an empty response and is expected to re-initialize the same request.
//!
//! The intent with a long polling fallback is to provide the ability for clients to operate in enviroments
//! that are hostile to plain TCP. Such environments are usually also relatively picky about killing idle HTTP connections,
//! so Procession's long polling implementation generally trades performance in order to operate more nicely with
//! proxies that may be configured with very severe timeouts.
//!
//! For improved performance, switch to the TCP server if possible.
//!
//! ## TCP
//!
//! The TCP worker API is implemented as a plain TCP connection where the client submits jobs by sending a client initiated payload,
//! and receives job assignments as server initiated payloads.
//!
//! The TCP API is used purely for submitting jobs and receiving job assignment.
//!
//! Any job management or statistics gathering that the client wishes to do is performed via the HTTP API;
//! the idea is that this simplifies both the client and server implementations by ensuring that when connected
//! over TCP both ends of the connection are solely concerned with one kind of task.

use std::{fmt::Display, net::SocketAddr, str::FromStr};

use axum::{Router, Server};
use clap::Parser;

use error_stack::{Context, IntoReport, Report, ResultExt};
use procession::api;

use tracing::info;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// HTTP server listen address, in host:port format.
    ///
    /// The HTTP server has a health check endpoint and serves the UI.
    /// It supports general job lifecycle management and reporting:
    /// - Management of submitted jobs
    /// - Statistics
    ///
    /// It also supports a similar worker API as clients:
    /// - Submit jobs
    /// - Receive jobs
    ///
    /// For more information, see the application level documentation at the root of this crate.
    #[clap(
        long = "listen-http-server",
        default_value = "0.0.0.0:3000",
        takes_value = true,
        parse(try_from_str = SocketAddr::from_str)
    )]
    listen_http: SocketAddr,

    /// Job server listen address, in host:port format.
    ///
    /// The job server is pure TCP and intended for maximal performance for worker clients,
    /// supporting submitting and receiving jobs.
    /// Job management is accomplished through the HTTP server.
    #[clap(
        long = "listen-job-server",
        default_value = "0.0.0.0:3001",
        takes_value = true,
        parse(try_from_str = SocketAddr::from_str)
    )]
    listen_job: SocketAddr,
}

#[derive(Debug)]
struct StartupError;

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start server")
    }
}

impl Context for StartupError {}

#[tokio::main]
async fn main() -> Result<(), Report<StartupError>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let listen_http = args.listen_http;

    let service_name = env!("CARGO_PKG_NAME");
    let service_version = env!("CARGO_PKG_VERSION");
    info!("👩🏻‍💻 Starting {service_name} {service_version} on {listen_http}",);

    let app = Router::new().merge(api::server());
    let bind = Server::try_bind(&listen_http)
        .into_report()
        .change_context(StartupError)?;

    let server = bind.serve(app.into_make_service());
    info!("✨ Serving {service_name} {service_version} on {listen_http}",);
    server.await.into_report().change_context(StartupError)?;

    info!("😴 Service exited cleanly, shutting down");
    Ok(())
}
