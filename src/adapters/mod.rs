#[cfg(feature = "ratatui")]
pub mod ratatui;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "css")]
pub mod css;

#[cfg(feature = "crossterm")]
pub mod crossterm;

#[cfg(feature = "owo-colors")]
pub mod owo_colors;

#[cfg(feature = "syntect")]
pub mod syntect;

#[cfg(feature = "egui")]
pub mod egui;

#[cfg(feature = "iced")]
pub mod iced;
