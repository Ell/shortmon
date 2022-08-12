pub mod capabilities;
pub mod input;
pub mod mccs;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::Monitor;
