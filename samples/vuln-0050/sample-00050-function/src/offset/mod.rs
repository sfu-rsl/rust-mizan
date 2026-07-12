// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! The time zone, which calculates offsets from the local time to UTC.
//!
//! There are four operations provided by the `TimeZone` trait:
//!
//! 1. Converting the local `NaiveDateTime` to `DateTime<Tz>`
//! 2. Converting the UTC `NaiveDateTime` to `DateTime<Tz>`
//! 3. Converting `DateTime<Tz>` to the local `NaiveDateTime`
//! 4. Constructing `DateTime<Tz>` objects from various offsets
//!
//! 1 is used for constructors. 2 is used for the `with_timezone` method of date and time types.
//! 3 is used for other methods, e.g. `year()` or `format()`, and provided by an associated type
//! which implements `Offset` (which then passed to `TimeZone` for actual implementations).
//! Technically speaking `TimeZone` has a total knowledge about given timescale,
//! but `Offset` is used as a cache to avoid the repeated conversion
//! and provides implementations for 1 and 3.
//! An `TimeZone` instance can be reconstructed from the corresponding `Offset` instance.

use core::fmt;

// use crate::format::{parse, ParseResult, Parsed, StrftimeItems};
use crate::naive::{NaiveDate, NaiveDateTime};
use crate::Weekday;
use crate::{Date, DateTime};

/// The conversion result from the local time to the timezone-aware datetime types.
#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum LocalResult<T> {
    /// Given local time representation is invalid.
    /// This can occur when, for example, the positive timezone transition.
    None,
    /// Given local time representation has a single unique result.
    Single(T),
    /// Given local time representation has multiple results and thus ambiguous.
    /// This can occur when, for example, the negative timezone transition.
    Ambiguous(T /*min*/, T /*max*/),
}

impl<T> LocalResult<T> {
    /// Returns `Some` only when the conversion result is unique, or `None` otherwise.
    pub fn single(self) -> Option<T> {
        match self {
            LocalResult::Single(t) => Some(t),
            _ => None,
        }
    }

    /// Maps a `LocalResult<T>` into `LocalResult<U>` with given function.
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> LocalResult<U> {
        match self {
            LocalResult::None => LocalResult::None,
            LocalResult::Single(v) => LocalResult::Single(f(v)),
            LocalResult::Ambiguous(min, max) => LocalResult::Ambiguous(f(min), f(max)),
        }
    }
}

impl<T: fmt::Debug> LocalResult<T> {
    /// Returns the single unique conversion result, or panics accordingly.
    pub fn unwrap(self) -> T {
        match self {
            LocalResult::None => panic!("No such local time"),
            LocalResult::Single(t) => t,
            LocalResult::Ambiguous(t1, t2) => {
                panic!("Ambiguous local time, ranging from {:?} to {:?}", t1, t2)
            }
        }
    }
}

/// The offset from the local time to UTC.
pub trait Offset: Sized + Clone + fmt::Debug {
    /// Returns the fixed offset from UTC to the local time stored.
    fn fix(&self) -> FixedOffset;
}

/// The time zone.
///
/// The methods here are the primarily constructors for [`Date`](../struct.Date.html) and
/// [`DateTime`](../struct.DateTime.html) types.
pub trait TimeZone: Sized + Clone {
    /// An associated offset type.
    /// This type is used to store the actual offset in date and time types.
    /// The original `TimeZone` value can be recovered via `TimeZone::from_offset`.
    type Offset: Offset;

    /// Reconstructs the time zone from the offset.
    fn from_offset(offset: &Self::Offset) -> Self;

    /// Creates the offset(s) for given local `NaiveDate` if possible.
    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Self::Offset>;

    /// Creates the offset(s) for given local `NaiveDateTime` if possible.
    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Self::Offset>;

    /// Converts the local `NaiveDate` to the timezone-aware `Date` if possible.
    fn from_local_date(&self, local: &NaiveDate) -> LocalResult<Date<Self>> {
        self.offset_from_local_date(local).map(|offset| {
            // since FixedOffset is within +/- 1 day, the date is never affected
            Date::from_utc(*local, offset)
        })
    }

    /// Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.
    fn from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<DateTime<Self>> {
        self.offset_from_local_datetime(local)
            .map(|offset| DateTime::from_utc(*local - offset.fix(), offset))
    }

    /// Creates the offset for given UTC `NaiveDate`. This cannot fail.
    fn offset_from_utc_date(&self, utc: &NaiveDate) -> Self::Offset;

    /// Creates the offset for given UTC `NaiveDateTime`. This cannot fail.
    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Self::Offset;

    /// Converts the UTC `NaiveDate` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    fn from_utc_date(&self, utc: &NaiveDate) -> Date<Self> {
        Date::from_utc(*utc, self.offset_from_utc_date(utc))
    }

    /// Converts the UTC `NaiveDateTime` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Self> {
        DateTime::from_utc(*utc, self.offset_from_utc_datetime(utc))
    }
}

mod fixed;
#[cfg(feature = "clock")]
mod local;
pub use self::fixed::FixedOffset;
#[cfg(feature = "clock")]
pub use self::local::Local;
