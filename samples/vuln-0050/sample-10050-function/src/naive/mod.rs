//! Date and time types unconcerned with timezones.
//!
//! They are primarily building blocks for other types
//! (e.g. [`TimeZone`](../offset/trait.TimeZone.html)),
//! but can be also used for the simpler date and time handling.

mod date;
pub(crate) mod datetime;
mod internals;
mod isoweek;
mod time;

#[allow(deprecated)]
pub use self::date::NaiveDate;

#[allow(deprecated)]
pub use self::datetime::NaiveDateTime;
pub use self::isoweek::IsoWeek;
pub use self::time::NaiveTime;
