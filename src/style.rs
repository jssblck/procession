use std::fmt::Display;

use owo_colors::{OwoColorize, Stream::Stdout};

pub fn url(input: &impl Display) -> String {
    colorize(input, |t| t.bright_purple().to_string())
}

pub fn constant<D: Display + ?Sized>(input: &D) -> String {
    colorize(input, |t| t.bright_blue().to_string())
}

fn colorize<T: Display + ?Sized>(input: &T, colorizer: impl Fn(&&T) -> String) -> String {
    format!("{}", input.if_supports_color(Stdout, colorizer))
}
