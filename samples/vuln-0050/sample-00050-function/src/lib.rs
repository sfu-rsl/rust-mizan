#![doc(html_root_url = "https://docs.rs/chrono/latest/")]
#![cfg_attr(feature = "bench", feature(test))] // lib stability features as per RFC #507
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
// #![deny(dead_code)]
// lints are added all the time, we test on 1.13
#![allow(unknown_lints)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(feature = "cargo-clippy", allow(
    renamed_and_removed_lints,
    // The explicit 'static lifetimes are still needed for rustc 1.13-16
    // backward compatibility, and this appeases clippy. If minimum rustc
    // becomes 1.17, should be able to remove this, those 'static lifetimes,
    // and use `static` in a lot of places `const` is used now.
    redundant_static_lifetimes,
    // Similarly, redundant_field_names lints on not using the
    // field-init-shorthand, which was stabilized in rust 1.17.
    redundant_field_names,
    // Changing trivially_copy_pass_by_ref would require an incompatible version
    // bump.
    trivially_copy_pass_by_ref,
    try_err,
    // Currently deprecated, we use the separate implementation to add docs
    // warning that putting a time in a hash table is probably a bad idea
    derive_hash_xor_eq,
))]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(all(feature = "std", not(feature = "alloc")))]
extern crate std as alloc;
#[cfg(any(feature = "std", test))]
extern crate std as core;

#[cfg(feature = "oldtime")]
extern crate time as oldtime;
#[cfg(not(feature = "oldtime"))]
mod oldtime;

#[cfg(feature = "clock")]
extern crate libc;
#[cfg(all(feature = "clock", windows))]
extern crate winapi;
#[cfg(all(
    feature = "clock",
    not(all(target_arch = "wasm32", not(target_os = "wasi"), feature = "wasmbind"))
))]
mod sys;

extern crate num_integer;
extern crate num_traits;

// this reexport is to aid the transition and should not be in the prelude!
pub use oldtime::Duration;

pub use date::Date;

pub use datetime::DateTime;

#[doc(no_inline)]
pub use naive::{IsoWeek, NaiveDate, NaiveDateTime, NaiveTime};
#[cfg(feature = "clock")]
#[doc(no_inline)]
pub use offset::Local;
#[doc(no_inline)]
pub use offset::{FixedOffset, LocalResult, Offset, TimeZone};

/// A convenience module appropriate for glob imports (`use chrono::prelude::*;`).
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::Date;
    #[doc(no_inline)]
    // pub use crate::SubsecRound;
    #[doc(no_inline)]
    pub use crate::DateTime;
    #[doc(no_inline)]
    pub use crate::FixedOffset;
    #[cfg(feature = "clock")]
    #[doc(no_inline)]
    pub use crate::Local;
    #[doc(no_inline)]
    pub use crate::{Datelike, Month, Timelike, Weekday};
    #[doc(no_inline)]
    pub use crate::{NaiveDate, NaiveDateTime, NaiveTime};
    #[doc(no_inline)]
    pub use crate::{Offset, TimeZone};
    #[cfg(feature = "unstable-locales")]
    #[doc(no_inline)]
    pub use Locale;
}

// useful throughout the codebase
macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None,
        }
    };
}

mod div;
pub mod offset;
pub mod naive {
    //! Date and time types unconcerned with timezones.
    //!
    //! They are primarily building blocks for other types
    //! (e.g. [`TimeZone`](../offset/trait.TimeZone.html)),
    //! but can be also used for the simpler date and time handling.

    mod date;
    mod datetime;
    mod internals;
    mod isoweek;
    mod time;

    pub use self::date::{NaiveDate, MAX_DATE, MIN_DATE};
    #[cfg(feature = "rustc-serialize")]
    #[allow(deprecated)]
    pub use self::datetime::rustc_serialize::TsSeconds;
    pub use self::datetime::{NaiveDateTime, MAX_DATETIME, MIN_DATETIME};
    pub use self::isoweek::IsoWeek;
    pub use self::time::NaiveTime;
}
mod date;
mod datetime;

/// The day of week.
///
/// The order of the days of week depends on the context.
/// (This is why this type does *not* implement `PartialOrd` or `Ord` traits.)
/// One should prefer `*_from_monday` or `*_from_sunday` methods to get the correct result.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
pub enum Weekday {
    /// Monday.
    Mon = 0,
    /// Tuesday.
    Tue = 1,
    /// Wednesday.
    Wed = 2,
    /// Thursday.
    Thu = 3,
    /// Friday.
    Fri = 4,
    /// Saturday.
    Sat = 5,
    /// Sunday.
    Sun = 6,
}

impl Weekday {
    /// The next day in the week.
    ///
    /// `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.succ()`: | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun` | `Mon`
    #[inline]
    pub fn succ(&self) -> Weekday {
        match *self {
            Weekday::Mon => Weekday::Tue,
            Weekday::Tue => Weekday::Wed,
            Weekday::Wed => Weekday::Thu,
            Weekday::Thu => Weekday::Fri,
            Weekday::Fri => Weekday::Sat,
            Weekday::Sat => Weekday::Sun,
            Weekday::Sun => Weekday::Mon,
        }
    }

    /// The previous day in the week.
    ///
    /// `w`:        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ----------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.pred()`: | `Sun` | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat`
    #[inline]
    pub fn pred(&self) -> Weekday {
        match *self {
            Weekday::Mon => Weekday::Sun,
            Weekday::Tue => Weekday::Mon,
            Weekday::Wed => Weekday::Tue,
            Weekday::Thu => Weekday::Wed,
            Weekday::Fri => Weekday::Thu,
            Weekday::Sat => Weekday::Fri,
            Weekday::Sun => Weekday::Sat,
        }
    }

    /// Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)
    ///
    /// `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.number_from_monday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 7
    #[inline]
    pub fn number_from_monday(&self) -> u32 {
        match *self {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 7,
        }
    }

    /// Returns a day-of-week number starting from Sunday = 1.
    ///
    /// `w`:                      | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// ------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.number_from_sunday()`: | 2     | 3     | 4     | 5     | 6     | 7     | 1
    #[inline]
    pub fn number_from_sunday(&self) -> u32 {
        match *self {
            Weekday::Mon => 2,
            Weekday::Tue => 3,
            Weekday::Wed => 4,
            Weekday::Thu => 5,
            Weekday::Fri => 6,
            Weekday::Sat => 7,
            Weekday::Sun => 1,
        }
    }

    /// Returns a day-of-week number starting from Monday = 0.
    ///
    /// `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.num_days_from_monday()`: | 0     | 1     | 2     | 3     | 4     | 5     | 6
    #[inline]
    pub fn num_days_from_monday(&self) -> u32 {
        match *self {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        }
    }

    /// Returns a day-of-week number starting from Sunday = 0.
    ///
    /// `w`:                        | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
    /// --------------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
    /// `w.num_days_from_sunday()`: | 1     | 2     | 3     | 4     | 5     | 6     | 0
    #[inline]
    pub fn num_days_from_sunday(&self) -> u32 {
        match *self {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        })
    }
}

/// Any weekday can be represented as an integer from 0 to 6, which equals to
/// [`Weekday::num_days_from_monday`](#method.num_days_from_monday) in this implementation.
/// Do not heavily depend on this though; use explicit methods whenever possible.
impl num_traits::FromPrimitive for Weekday {
    #[inline]
    fn from_i64(n: i64) -> Option<Weekday> {
        match n {
            0 => Some(Weekday::Mon),
            1 => Some(Weekday::Tue),
            2 => Some(Weekday::Wed),
            3 => Some(Weekday::Thu),
            4 => Some(Weekday::Fri),
            5 => Some(Weekday::Sat),
            6 => Some(Weekday::Sun),
            _ => None,
        }
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Weekday> {
        match n {
            0 => Some(Weekday::Mon),
            1 => Some(Weekday::Tue),
            2 => Some(Weekday::Wed),
            3 => Some(Weekday::Thu),
            4 => Some(Weekday::Fri),
            5 => Some(Weekday::Sat),
            6 => Some(Weekday::Sun),
            _ => None,
        }
    }
}

use core::fmt;

/// An error resulting from reading `Weekday` value with `FromStr`.
#[derive(Clone, PartialEq)]
pub struct ParseWeekdayError {
    _dummy: (),
}

impl fmt::Debug for ParseWeekdayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseWeekdayError {{ .. }}")
    }
}

/// The month of the year.
///
/// This enum is just a convenience implementation.
/// The month in dates created by DateLike objects does not return this enum.
///
/// It is possible to convert from a date to a month independently
/// ```
/// # extern crate num_traits;
/// use num_traits::FromPrimitive;
/// use chrono::prelude::*;
/// let date = Utc.ymd(2019, 10, 28).and_hms(9, 10, 11);
/// // `2019-10-28T09:10:11Z`
/// let month = Month::from_u32(date.month());
/// assert_eq!(month, Some(Month::October))
/// ```
/// Or from a Month to an integer usable by dates
/// ```
/// # use chrono::prelude::*;
/// let month = Month::January;
/// let dt = Utc.ymd(2019, month.number_from_month(), 28).and_hms(9, 10, 11);
/// assert_eq!((dt.year(), dt.month(), dt.day()), (2019, 1, 28));
/// ```
/// Allows mapping from and to month, from 1-January to 12-December.
/// Can be Serialized/Deserialized with serde
// Actual implementation is zero-indexed, API intended as 1-indexed for more intuitive behavior.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
pub enum Month {
    /// January
    January = 0,
    /// February
    February = 1,
    /// March
    March = 2,
    /// April
    April = 3,
    /// May
    May = 4,
    /// June
    June = 5,
    /// July
    July = 6,
    /// August
    August = 7,
    /// September
    September = 8,
    /// October
    October = 9,
    /// November
    November = 10,
    /// December
    December = 11,
}

impl Month {
    /// The next month.
    ///
    /// `m`:        | `January`  | `February` | `...` | `December`
    /// ----------- | ---------  | ---------- | --- | ---------
    /// `m.succ()`: | `February` | `March`    | `...` | `January`
    #[inline]
    pub fn succ(&self) -> Month {
        match *self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }

    /// The previous month.
    ///
    /// `m`:        | `January`  | `February` | `...` | `December`
    /// ----------- | ---------  | ---------- | --- | ---------
    /// `m.succ()`: | `December` | `January`  | `...` | `November`
    #[inline]
    pub fn pred(&self) -> Month {
        match *self {
            Month::January => Month::December,
            Month::February => Month::January,
            Month::March => Month::February,
            Month::April => Month::March,
            Month::May => Month::April,
            Month::June => Month::May,
            Month::July => Month::June,
            Month::August => Month::July,
            Month::September => Month::August,
            Month::October => Month::September,
            Month::November => Month::October,
            Month::December => Month::November,
        }
    }

    /// Returns a month-of-year number starting from January = 1.
    ///
    /// `m`:                     | `January` | `February` | `...` | `December`
    /// -------------------------| --------- | ---------- | --- | -----
    /// `m.number_from_month()`: | 1         | 2          | `...` | 12
    #[inline]
    pub fn number_from_month(&self) -> u32 {
        match *self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }

    /// Get the name of the month
    ///
    /// ```
    /// use chrono::Month;
    ///
    /// assert_eq!(Month::January.name(), "January")
    /// ```
    pub fn name(&self) -> &'static str {
        match *self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }
}

impl num_traits::FromPrimitive for Month {
    /// Returns an Option<Month> from a i64, assuming a 1-index, January = 1.
    ///
    /// `Month::from_i64(n: i64)`: | `1`                  | `2`                   | ... | `12`
    /// ---------------------------| -------------------- | --------------------- | ... | -----
    /// ``:                        | Some(Month::January) | Some(Month::February) | ... | Some(Month::December)

    #[inline]
    fn from_u64(n: u64) -> Option<Month> {
        Self::from_u32(n as u32)
    }

    #[inline]
    fn from_i64(n: i64) -> Option<Month> {
        Self::from_u32(n as u32)
    }

    #[inline]
    fn from_u32(n: u32) -> Option<Month> {
        match n {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }
}

/// An error resulting from reading `<Month>` value with `FromStr`.
#[derive(Clone, PartialEq)]
pub struct ParseMonthError {
    _dummy: (),
}

impl fmt::Debug for ParseMonthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseMonthError {{ .. }}")
    }
}

/// The common set of methods for date component.
pub trait Datelike: Sized {
    /// Returns the year number in the [calendar date](./naive/struct.NaiveDate.html#calendar-date).
    fn year(&self) -> i32;

    /// Returns the absolute year number starting from 1 with a boolean flag,
    /// which is false when the year predates the epoch (BCE/BC) and true otherwise (CE/AD).
    #[inline]
    fn year_ce(&self) -> (bool, u32) {
        let year = self.year();
        if year < 1 {
            (false, (1 - year) as u32)
        } else {
            (true, year as u32)
        }
    }

    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    fn month(&self) -> u32;

    /// Returns the month number starting from 0.
    ///
    /// The return value ranges from 0 to 11.
    fn month0(&self) -> u32;

    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    fn day(&self) -> u32;

    /// Returns the day of month starting from 0.
    ///
    /// The return value ranges from 0 to 30. (The last day of month differs by months.)
    fn day0(&self) -> u32;

    /// Returns the day of year starting from 1.
    ///
    /// The return value ranges from 1 to 366. (The last day of year differs by years.)
    fn ordinal(&self) -> u32;

    /// Returns the day of year starting from 0.
    ///
    /// The return value ranges from 0 to 365. (The last day of year differs by years.)
    fn ordinal0(&self) -> u32;

    /// Returns the day of week.
    fn weekday(&self) -> Weekday;

    /// Returns the ISO week.
    fn iso_week(&self) -> IsoWeek;

    /// Makes a new value with the year number changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_year(&self, year: i32) -> Option<Self>;

    /// Makes a new value with the month number (starting from 1) changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_month(&self, month: u32) -> Option<Self>;

    /// Makes a new value with the month number (starting from 0) changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_month0(&self, month0: u32) -> Option<Self>;

    /// Makes a new value with the day of month (starting from 1) changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_day(&self, day: u32) -> Option<Self>;

    /// Makes a new value with the day of month (starting from 0) changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_day0(&self, day0: u32) -> Option<Self>;

    /// Makes a new value with the day of year (starting from 1) changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_ordinal(&self, ordinal: u32) -> Option<Self>;

    /// Makes a new value with the day of year (starting from 0) changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_ordinal0(&self, ordinal0: u32) -> Option<Self>;

    /// Counts the days in the proleptic Gregorian calendar, with January 1, Year 1 (CE) as day 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// assert_eq!(NaiveDate::from_ymd(1970, 1, 1).num_days_from_ce(), 719_163);
    /// assert_eq!(NaiveDate::from_ymd(2, 1, 1).num_days_from_ce(), 366);
    /// assert_eq!(NaiveDate::from_ymd(1, 1, 1).num_days_from_ce(), 1);
    /// assert_eq!(NaiveDate::from_ymd(0, 1, 1).num_days_from_ce(), -365);
    /// ```
    fn num_days_from_ce(&self) -> i32 {
        // See test_num_days_from_ce_against_alternative_impl below for a more straightforward
        // implementation.

        // we know this wouldn't overflow since year is limited to 1/2^13 of i32's full range.
        let mut year = self.year() - 1;
        let mut ndays = 0;
        if year < 0 {
            let excess = 1 + (-year) / 400;
            year += excess * 400;
            ndays -= excess * 146_097;
        }
        let div_100 = year / 100;
        ndays += ((year * 1461) >> 2) - div_100 + (div_100 >> 2);
        ndays + self.ordinal() as i32
    }
}

/// The common set of methods for time component.
pub trait Timelike: Sized {
    /// Returns the hour number from 0 to 23.
    fn hour(&self) -> u32;

    /// Returns the hour number from 1 to 12 with a boolean flag,
    /// which is false for AM and true for PM.
    #[inline]
    fn hour12(&self) -> (bool, u32) {
        let hour = self.hour();
        let mut hour12 = hour % 12;
        if hour12 == 0 {
            hour12 = 12;
        }
        (hour >= 12, hour12)
    }

    /// Returns the minute number from 0 to 59.
    fn minute(&self) -> u32;

    /// Returns the second number from 0 to 59.
    fn second(&self) -> u32;

    /// Returns the number of nanoseconds since the whole non-leap second.
    /// The range from 1,000,000,000 to 1,999,999,999 represents
    /// the [leap second](./naive/struct.NaiveTime.html#leap-second-handling).
    fn nanosecond(&self) -> u32;

    /// Makes a new value with the hour number changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_hour(&self, hour: u32) -> Option<Self>;

    /// Makes a new value with the minute number changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    fn with_minute(&self, min: u32) -> Option<Self>;

    /// Makes a new value with the second number changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    /// As with the [`second`](#tymethod.second) method,
    /// the input range is restricted to 0 through 59.
    fn with_second(&self, sec: u32) -> Option<Self>;

    /// Makes a new value with nanoseconds since the whole non-leap second changed.
    ///
    /// Returns `None` when the resulting value would be invalid.
    /// As with the [`nanosecond`](#tymethod.nanosecond) method,
    /// the input range can exceed 1,000,000,000 for leap seconds.
    fn with_nanosecond(&self, nano: u32) -> Option<Self>;

    /// Returns the number of non-leap seconds past the last midnight.
    #[inline]
    fn num_seconds_from_midnight(&self) -> u32 {
        self.hour() * 3600 + self.minute() * 60 + self.second()
    }
}
