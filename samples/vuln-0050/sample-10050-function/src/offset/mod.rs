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

use crate::{Date, DateTime};

mod fixed;
pub use self::fixed::FixedOffset;

#[cfg(feature = "clock")]
mod local;
#[cfg(feature = "clock")]
pub use self::local::Local;

mod utc;
pub use self::utc::Utc;

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

    /// Returns `Some` for the earliest possible conversion result, or `None` if none.
    pub fn earliest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(t, _) => Some(t),
            _ => None,
        }
    }

    /// Returns `Some` for the latest possible conversion result, or `None` if none.
    pub fn latest(self) -> Option<T> {
        match self {
            LocalResult::Single(t) | LocalResult::Ambiguous(_, t) => Some(t),
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

    /// Makes a new `Date` from year, month, day and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Panics on the out-of-range date, invalid month and/or day.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.ymd(2015, 5, 15).to_string(), "2015-05-15UTC");
    /// ```
    fn ymd(&self, year: i32, month: u32, day: u32) -> Date<Self> {
        self.ymd_opt(year, month, day).unwrap()
    }

    /// Makes a new `Date` from year, month, day and the current time zone.
    /// This assumes the proleptic Gregorian calendar, with the year 0 being 1 BCE.
    ///
    /// The time zone normally does not affect the date (unless it is between UTC-24 and UTC+24),
    /// but it will propagate to the `DateTime` values constructed via this date.
    ///
    /// Returns `None` on the out-of-range date, invalid month and/or day.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, LocalResult, TimeZone};
    ///
    /// assert_eq!(Utc.ymd_opt(2015, 5, 15).unwrap().to_string(), "2015-05-15UTC");
    /// assert_eq!(Utc.ymd_opt(2000, 0, 0), LocalResult::None);
    /// ```
    fn ymd_opt(&self, year: i32, month: u32, day: u32) -> LocalResult<Date<Self>> {
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(d) => self.from_local_date(&d),
            None => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the number of non-leap seconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// Panics on the out-of-range number of seconds and/or invalid nanosecond,
    /// for a non-panicking version see [`timestamp_opt`](#method.timestamp_opt).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp(1431648000, 0).to_string(), "2015-05-15 00:00:00 UTC");
    /// ```
    fn timestamp(&self, secs: i64, nsecs: u32) -> DateTime<Self> {
        self.timestamp_opt(secs, nsecs).unwrap()
    }

    /// Makes a new `DateTime` from the number of non-leap seconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp")
    /// and the number of nanoseconds since the last whole non-leap second.
    ///
    /// Returns `LocalResult::None` on out-of-range number of seconds and/or
    /// invalid nanosecond, otherwise always returns `LocalResult::Single`.
    fn timestamp_opt(&self, secs: i64, nsecs: u32) -> LocalResult<DateTime<Self>> {
        match NaiveDateTime::from_timestamp_opt(secs, nsecs) {
            Some(dt) => LocalResult::Single(self.from_utc_datetime(&dt)),
            None => LocalResult::None,
        }
    }

    /// Makes a new `DateTime` from the number of non-leap milliseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// Panics on out-of-range number of milliseconds for a non-panicking
    /// version see [`timestamp_millis_opt`](#method.timestamp_millis_opt).
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp_millis(1431648000).timestamp(), 1431648);
    /// ```
    fn timestamp_millis(&self, millis: i64) -> DateTime<Self> {
        self.timestamp_millis_opt(millis).unwrap()
    }

    /// Makes a new `DateTime` from the number of non-leap milliseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    ///
    /// Returns `LocalResult::None` on out-of-range number of milliseconds
    /// and/or invalid nanosecond, otherwise always returns
    /// `LocalResult::Single`.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone, LocalResult};
    /// match Utc.timestamp_millis_opt(1431648000) {
    ///     LocalResult::Single(dt) => assert_eq!(dt.timestamp(), 1431648),
    ///     _ => panic!("Incorrect timestamp_millis"),
    /// };
    /// ```
    fn timestamp_millis_opt(&self, millis: i64) -> LocalResult<DateTime<Self>> {
        let (mut secs, mut millis) = (millis / 1000, millis % 1000);
        if millis < 0 {
            secs -= 1;
            millis += 1000;
        }
        self.timestamp_opt(secs, millis as u32 * 1_000_000)
    }

    /// Makes a new `DateTime` from the number of non-leap nanoseconds
    /// since January 1, 1970 0:00:00 UTC (aka "UNIX timestamp").
    ///
    /// Unlike [`timestamp_millis`](#method.timestamp_millis), this never
    /// panics.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    ///
    /// assert_eq!(Utc.timestamp_nanos(1431648000000000).timestamp(), 1431648);
    /// ```
    fn timestamp_nanos(&self, nanos: i64) -> DateTime<Self> {
        let (mut secs, mut nanos) = (nanos / 1_000_000_000, nanos % 1_000_000_000);
        if nanos < 0 {
            secs -= 1;
            nanos += 1_000_000_000;
        }
        self.timestamp_opt(secs, nanos as u32).unwrap()
    }

    /// Parses a string with the specified format string and returns a
    /// `DateTime` with the current offset.
    ///
    /// See the [`crate::format::strftime`] module on the
    /// supported escape sequences.
    ///
    /// If the to-be-parsed string includes an offset, it *must* match the
    /// offset of the TimeZone, otherwise an error will be returned.
    ///
    /// See also [`DateTime::parse_from_str`] which gives a [`DateTime`] with
    /// parsed [`FixedOffset`].
    // fn datetime_from_str(&self, s: &str, fmt: &str) -> ParseResult<DateTime<Self>> {
    //     let mut parsed = Parsed::new();
    //     parse(&mut parsed, s, StrftimeItems::new(fmt))?;
    //     parsed.to_datetime_with_timezone(self)
    // }

    /// Reconstructs the time zone from the offset.
    fn from_offset(offset: &Self::Offset) -> Self;

    /// Creates the offset(s) for given local `NaiveDate` if possible.
    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Self::Offset>;

    /// Creates the offset(s) for given local `NaiveDateTime` if possible.
    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Self::Offset>;

    /// Converts the local `NaiveDate` to the timezone-aware `Date` if possible.
    #[allow(clippy::wrong_self_convention)]
    fn from_local_date(&self, local: &NaiveDate) -> LocalResult<Date<Self>> {
        self.offset_from_local_date(local).map(|offset| {
            // since FixedOffset is within +/- 1 day, the date is never affected
            Date::from_utc(*local, offset)
        })
    }

    /// Converts the local `NaiveDateTime` to the timezone-aware `DateTime` if possible.
    #[allow(clippy::wrong_self_convention)]
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
    #[allow(clippy::wrong_self_convention)]
    fn from_utc_date(&self, utc: &NaiveDate) -> Date<Self> {
        Date::from_utc(*utc, self.offset_from_utc_date(utc))
    }

    /// Converts the UTC `NaiveDateTime` to the local time.
    /// The UTC is continuous and thus this cannot fail (but can give the duplicate local time).
    #[allow(clippy::wrong_self_convention)]
    fn from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Self> {
        DateTime::from_utc(*utc, self.offset_from_utc_datetime(utc))
    }
}
