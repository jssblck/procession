//! Style helpers for command line output.

use std::fmt::Display;

use axum::http::StatusCode;
use owo_colors::{OwoColorize, Stream::Stdout};

/// Style the text as a URL.
pub fn url(input: &impl Display) -> String {
    colorize(input, |t| t.bright_purple().to_string())
}

/// Style the text as a constant.
pub fn constant<D: Display + ?Sized>(input: &D) -> String {
    colorize(input, |t| t.bright_blue().to_string())
}

/// Style the text as a status code:
/// - Successful: Green
/// - Client errors: Yellow
/// - Server errors: Red
pub fn status_code(input: StatusCode) -> String {
    let status_code = input.to_string();
    match input.as_u16() {
        0..=399 => colorize(&status_code, |t| t.bright_green().to_string()),
        400..=499 => colorize(&status_code, |t| t.bright_yellow().to_string()),
        500.. => colorize(&status_code, |t| t.bright_red().to_string()),
    }
}

fn colorize<T: Display + ?Sized>(input: &T, colorizer: impl Fn(&&T) -> String) -> String {
    format!("{}", input.if_supports_color(Stdout, colorizer))
}
