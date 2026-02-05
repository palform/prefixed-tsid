#![doc = include_str!("../README.md")]

pub mod resources;
pub mod tsid;

#[cfg(feature = "diesel")]
pub mod diesel;
#[cfg(feature = "oasgen")]
pub mod oasgen;
#[cfg(feature = "rocket")]
pub mod rocket;
#[cfg(feature = "schemars")]
pub mod schemars;
#[cfg(feature = "sea-orm")]
pub mod sea_orm;
#[cfg(feature = "serde")]
pub mod serde;
