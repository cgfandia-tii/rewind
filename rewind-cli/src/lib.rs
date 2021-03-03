
use std::str::FromStr;

pub use rewind_core::{fuzz, mem, mutation, trace, trace::Tracer};
pub use rewind_bochs::BochsTracer;

#[cfg(windows)]
pub use rewind_whvp::WhvpTracer;

pub mod helpers;
pub mod cli;

pub use crate::cli::Rewind;

pub use color_eyre;
#[derive(Debug)]
pub enum BackendType {
    #[cfg(windows)]
    Whvp,
    Bochs
}

impl FromStr for BackendType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bochs" => Ok(Self::Bochs),
            #[cfg(windows)]
            "whvp" => Ok(Self::Whvp),
            _ => Err("no match"),
        }
    }
}

impl std::fmt::Display for BackendType {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Bochs => {
                f.write_str("bochs")
            },
            #[cfg(windows)]
            Self::Whvp => {
                f.write_str("whvp")
            }
        }
    }
}

