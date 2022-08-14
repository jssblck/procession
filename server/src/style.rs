//! Style helpers for command line output.

use std::fmt::Display;

use owo_colors::{OwoColorize, Stream::Stdout};

/// Style the text as a URL.
pub fn url(input: &impl Display) -> String {
    colorize(input, |t| t.bright_purple().to_string())
}

/// Style the text as a constant.
pub fn constant<D: Display + ?Sized>(input: &D) -> String {
    colorize(input, |t| t.bright_blue().to_string())
}

fn colorize<T: Display + ?Sized>(input: &T, colorizer: impl Fn(&&T) -> String) -> String {
    format!("{}", input.if_supports_color(Stdout, colorizer))
}
