// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! ISO 8601 date and time without timezone.

#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::naive::date::{MAX_DATE, MIN_DATE};
use crate::naive::time::{MAX_TIME, MIN_TIME};
use crate::naive::{IsoWeek, NaiveDate, NaiveTime};
use crate::{Datelike, Timelike, Weekday};
use core::fmt;
#[cfg(any(feature = "alloc", feature = "std", test))]
use core::ops::Add;
use oldtime::Duration as OldDuration;

/// The tight upper bound guarantees that a duration with `|Duration| >= 2^MAX_SECS_BITS`
/// will always overflow the addition with any date and time type.
///
/// So why is this needed? `Duration::seconds(rhs)` may overflow, and we don't have
/// an alternative returning `Option` or `Result`. Thus we need some early bound to avoid
/// touching that call when we are already sure that it WILL overflow...
const MAX_SECS_BITS: usize = 44;

/// The minimum possible `NaiveDateTime`.
pub const MIN_DATETIME: NaiveDateTime = NaiveDateTime {
    date: MIN_DATE,
    time: MIN_TIME,
};
/// The maximum possible `NaiveDateTime`.
pub const MAX_DATETIME: NaiveDateTime = NaiveDateTime {
    date: MAX_DATE,
    time: MAX_TIME,
};

/// ISO 8601 combined date and time without timezone.
///
/// # Example
///
/// `NaiveDateTime` is commonly created from [`NaiveDate`](./struct.NaiveDate.html).
///
/// ~~~~
/// use chrono::{NaiveDate, NaiveDateTime};
///
/// let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
/// # let _ = dt;
/// ~~~~
///
/// You can use typical [date-like](../trait.Datelike.html) and
/// [time-like](../trait.Timelike.html) methods,
/// provided that relevant traits are in the scope.
///
/// ~~~~
/// # use chrono::{NaiveDate, NaiveDateTime};
/// # let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
/// use chrono::{Datelike, Timelike, Weekday};
///
/// assert_eq!(dt.weekday(), Weekday::Fri);
/// assert_eq!(dt.num_seconds_from_midnight(), 33011);
/// ~~~~
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct NaiveDateTime {
    date: NaiveDate,
    time: NaiveTime,
}

impl NaiveDateTime {
    /// Makes a new `NaiveDateTime` from date and time components.
    /// Equivalent to [`date.and_time(time)`](./struct.NaiveDate.html#method.and_time)
    /// and many other helper constructors on `NaiveDate`.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    ///
    /// let d = NaiveDate::from_ymd(2015, 6, 3);
    /// let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
    ///
    /// let dt = NaiveDateTime::new(d, t);
    /// assert_eq!(dt.date(), d);
    /// assert_eq!(dt.time(), t);
    /// ~~~~
    #[inline]
    pub fn new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime {
        NaiveDateTime {
            date: date,
            time: time,
        }
    }

    /// Retrieves a date component.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::NaiveDate;
    ///
    /// let dt = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
    /// assert_eq!(dt.date(), NaiveDate::from_ymd(2016, 7, 8));
    /// ~~~~
    #[inline]
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Adds given `Duration` to the current date and time.
    ///
    /// As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),
    /// the addition assumes that **there is no leap second ever**,
    /// except when the `NaiveDateTime` itself represents a leap second
    /// in which case the assumption becomes that **there is exactly a single leap second ever**.
    ///
    /// Returns `None` when it will result in overflow.
    ///
    /// # Example
    ///
    /// ~~~~
    /// # extern crate chrono; fn main() {
    /// use chrono::{Duration, NaiveDate};
    ///
    /// let from_ymd = NaiveDate::from_ymd;
    ///
    /// let d = from_ymd(2016, 7, 8);
    /// let hms = |h, m, s| d.and_hms(h, m, s);
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(Duration::zero()),
    ///            Some(hms(3, 5, 7)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(Duration::seconds(1)),
    ///            Some(hms(3, 5, 8)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(Duration::seconds(-1)),
    ///            Some(hms(3, 5, 6)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(Duration::seconds(3600 + 60)),
    ///            Some(hms(4, 6, 7)));
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(Duration::seconds(86_400)),
    ///            Some(from_ymd(2016, 7, 9).and_hms(3, 5, 7)));
    ///
    /// let hmsm = |h, m, s, milli| d.and_hms_milli(h, m, s, milli);
    /// assert_eq!(hmsm(3, 5, 7, 980).checked_add_signed(Duration::milliseconds(450)),
    ///            Some(hmsm(3, 5, 8, 430)));
    /// # }
    /// ~~~~
    ///
    /// Overflow returns `None`.
    ///
    /// ~~~~
    /// # extern crate chrono; fn main() {
    /// # use chrono::{Duration, NaiveDate};
    /// # let hms = |h, m, s| NaiveDate::from_ymd(2016, 7, 8).and_hms(h, m, s);
    /// assert_eq!(hms(3, 5, 7).checked_add_signed(Duration::days(1_000_000_000)), None);
    /// # }
    /// ~~~~
    ///
    /// Leap seconds are handled,
    /// but the addition assumes that it is the only leap second happened.
    ///
    /// ~~~~
    /// # extern crate chrono; fn main() {
    /// # use chrono::{Duration, NaiveDate};
    /// # let from_ymd = NaiveDate::from_ymd;
    /// # let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli(h, m, s, milli);
    /// let leap = hmsm(3, 5, 59, 1_300);
    /// assert_eq!(leap.checked_add_signed(Duration::zero()),
    ///            Some(hmsm(3, 5, 59, 1_300)));
    /// assert_eq!(leap.checked_add_signed(Duration::milliseconds(-500)),
    ///            Some(hmsm(3, 5, 59, 800)));
    /// assert_eq!(leap.checked_add_signed(Duration::milliseconds(500)),
    ///            Some(hmsm(3, 5, 59, 1_800)));
    /// assert_eq!(leap.checked_add_signed(Duration::milliseconds(800)),
    ///            Some(hmsm(3, 6, 0, 100)));
    /// assert_eq!(leap.checked_add_signed(Duration::seconds(10)),
    ///            Some(hmsm(3, 6, 9, 300)));
    /// assert_eq!(leap.checked_add_signed(Duration::seconds(-10)),
    ///            Some(hmsm(3, 5, 50, 300)));
    /// assert_eq!(leap.checked_add_signed(Duration::days(1)),
    ///            Some(from_ymd(2016, 7, 9).and_hms_milli(3, 5, 59, 300)));
    /// # }
    /// ~~~~
    pub fn checked_add_signed(self, rhs: OldDuration) -> Option<NaiveDateTime> {
        let (time, rhs) = self.time.overflowing_add_signed(rhs);

        // early checking to avoid overflow in OldDuration::seconds
        if rhs <= (-1 << MAX_SECS_BITS) || rhs >= (1 << MAX_SECS_BITS) {
            return None;
        }

        let date = try_opt!(self.date.checked_add_signed(OldDuration::seconds(rhs)));
        Some(NaiveDateTime {
            date: date,
            time: time,
        })
    }
}

impl Datelike for NaiveDateTime {
    /// Returns the year number in the [calendar date](./index.html#calendar-date).
    ///
    /// See also the [`NaiveDate::year`](./struct.NaiveDate.html#method.year) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.year(), 2015);
    /// ~~~~
    #[inline]
    fn year(&self) -> i32 {
        self.date.year()
    }

    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    ///
    /// See also the [`NaiveDate::month`](./struct.NaiveDate.html#method.month) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.month(), 9);
    /// ~~~~
    #[inline]
    fn month(&self) -> u32 {
        self.date.month()
    }

    /// Returns the month number starting from 0.
    ///
    /// The return value ranges from 0 to 11.
    ///
    /// See also the [`NaiveDate::month0`](./struct.NaiveDate.html#method.month0) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.month0(), 8);
    /// ~~~~
    #[inline]
    fn month0(&self) -> u32 {
        self.date.month0()
    }

    /// Returns the day of month starting from 1.
    ///
    /// The return value ranges from 1 to 31. (The last day of month differs by months.)
    ///
    /// See also the [`NaiveDate::day`](./struct.NaiveDate.html#method.day) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.day(), 25);
    /// ~~~~
    #[inline]
    fn day(&self) -> u32 {
        self.date.day()
    }

    /// Returns the day of month starting from 0.
    ///
    /// The return value ranges from 0 to 30. (The last day of month differs by months.)
    ///
    /// See also the [`NaiveDate::day0`](./struct.NaiveDate.html#method.day0) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.day0(), 24);
    /// ~~~~
    #[inline]
    fn day0(&self) -> u32 {
        self.date.day0()
    }

    /// Returns the day of year starting from 1.
    ///
    /// The return value ranges from 1 to 366. (The last day of year differs by years.)
    ///
    /// See also the [`NaiveDate::ordinal`](./struct.NaiveDate.html#method.ordinal) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.ordinal(), 268);
    /// ~~~~
    #[inline]
    fn ordinal(&self) -> u32 {
        self.date.ordinal()
    }

    /// Returns the day of year starting from 0.
    ///
    /// The return value ranges from 0 to 365. (The last day of year differs by years.)
    ///
    /// See also the [`NaiveDate::ordinal0`](./struct.NaiveDate.html#method.ordinal0) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.ordinal0(), 267);
    /// ~~~~
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.date.ordinal0()
    }

    /// Returns the day of week.
    ///
    /// See also the [`NaiveDate::weekday`](./struct.NaiveDate.html#method.weekday) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike, Weekday};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.weekday(), Weekday::Fri);
    /// ~~~~
    #[inline]
    fn weekday(&self) -> Weekday {
        self.date.weekday()
    }

    #[inline]
    fn iso_week(&self) -> IsoWeek {
        self.date.iso_week()
    }

    /// Makes a new `NaiveDateTime` with the year number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_year`](./struct.NaiveDate.html#method.with_year) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 25).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_year(2016), Some(NaiveDate::from_ymd(2016, 9, 25).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_year(-308), Some(NaiveDate::from_ymd(-308, 9, 25).and_hms(12, 34, 56)));
    /// ~~~~
    #[inline]
    fn with_year(&self, year: i32) -> Option<NaiveDateTime> {
        self.date
            .with_year(year)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the month number (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_month`](./struct.NaiveDate.html#method.with_month) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 30).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_month(10), Some(NaiveDate::from_ymd(2015, 10, 30).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_month(13), None); // no month 13
    /// assert_eq!(dt.with_month(2), None); // no February 30
    /// ~~~~
    #[inline]
    fn with_month(&self, month: u32) -> Option<NaiveDateTime> {
        self.date
            .with_month(month)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the month number (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_month0`](./struct.NaiveDate.html#method.with_month0) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 30).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_month0(9), Some(NaiveDate::from_ymd(2015, 10, 30).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_month0(12), None); // no month 13
    /// assert_eq!(dt.with_month0(1), None); // no February 30
    /// ~~~~
    #[inline]
    fn with_month0(&self, month0: u32) -> Option<NaiveDateTime> {
        self.date
            .with_month0(month0)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the day of month (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_day`](./struct.NaiveDate.html#method.with_day) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_day(30), Some(NaiveDate::from_ymd(2015, 9, 30).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_day(31), None); // no September 31
    /// ~~~~
    #[inline]
    fn with_day(&self, day: u32) -> Option<NaiveDateTime> {
        self.date
            .with_day(day)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the day of month (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_day0`](./struct.NaiveDate.html#method.with_day0) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_day0(29), Some(NaiveDate::from_ymd(2015, 9, 30).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_day0(30), None); // no September 31
    /// ~~~~
    #[inline]
    fn with_day0(&self, day0: u32) -> Option<NaiveDateTime> {
        self.date
            .with_day0(day0)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the day of year (starting from 1) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_ordinal`](./struct.NaiveDate.html#method.with_ordinal) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_ordinal(60),
    ///            Some(NaiveDate::from_ymd(2015, 3, 1).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_ordinal(366), None); // 2015 had only 365 days
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 9, 8).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_ordinal(60),
    ///            Some(NaiveDate::from_ymd(2016, 2, 29).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_ordinal(366),
    ///            Some(NaiveDate::from_ymd(2016, 12, 31).and_hms(12, 34, 56)));
    /// ~~~~
    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDateTime> {
        self.date
            .with_ordinal(ordinal)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the day of year (starting from 0) changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveDate::with_ordinal0`](./struct.NaiveDate.html#method.with_ordinal0) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Datelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd(2015, 3, 1).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_ordinal0(365), None); // 2015 had only 365 days
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2016, 9, 8).and_hms(12, 34, 56);
    /// assert_eq!(dt.with_ordinal0(59),
    ///            Some(NaiveDate::from_ymd(2016, 2, 29).and_hms(12, 34, 56)));
    /// assert_eq!(dt.with_ordinal0(365),
    ///            Some(NaiveDate::from_ymd(2016, 12, 31).and_hms(12, 34, 56)));
    /// ~~~~
    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDateTime> {
        self.date
            .with_ordinal0(ordinal0)
            .map(|d| NaiveDateTime { date: d, ..*self })
    }
}

impl Timelike for NaiveDateTime {
    /// Returns the hour number from 0 to 23.
    ///
    /// See also the [`NaiveTime::hour`](./struct.NaiveTime.html#method.hour) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.hour(), 12);
    /// ~~~~
    #[inline]
    fn hour(&self) -> u32 {
        self.time.hour()
    }

    /// Returns the minute number from 0 to 59.
    ///
    /// See also the [`NaiveTime::minute`](./struct.NaiveTime.html#method.minute) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.minute(), 34);
    /// ~~~~
    #[inline]
    fn minute(&self) -> u32 {
        self.time.minute()
    }

    /// Returns the second number from 0 to 59.
    ///
    /// See also the [`NaiveTime::second`](./struct.NaiveTime.html#method.second) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.second(), 56);
    /// ~~~~
    #[inline]
    fn second(&self) -> u32 {
        self.time.second()
    }

    /// Returns the number of nanoseconds since the whole non-leap second.
    /// The range from 1,000,000,000 to 1,999,999,999 represents
    /// the [leap second](./struct.NaiveTime.html#leap-second-handling).
    ///
    /// See also the
    /// [`NaiveTime::nanosecond`](./struct.NaiveTime.html#method.nanosecond) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.nanosecond(), 789_000_000);
    /// ~~~~
    #[inline]
    fn nanosecond(&self) -> u32 {
        self.time.nanosecond()
    }

    /// Makes a new `NaiveDateTime` with the hour number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveTime::with_hour`](./struct.NaiveTime.html#method.with_hour) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.with_hour(7),
    ///            Some(NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(7, 34, 56, 789)));
    /// assert_eq!(dt.with_hour(24), None);
    /// ~~~~
    #[inline]
    fn with_hour(&self, hour: u32) -> Option<NaiveDateTime> {
        self.time
            .with_hour(hour)
            .map(|t| NaiveDateTime { time: t, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the minute number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    ///
    /// See also the
    /// [`NaiveTime::with_minute`](./struct.NaiveTime.html#method.with_minute) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.with_minute(45),
    ///            Some(NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 45, 56, 789)));
    /// assert_eq!(dt.with_minute(60), None);
    /// ~~~~
    #[inline]
    fn with_minute(&self, min: u32) -> Option<NaiveDateTime> {
        self.time
            .with_minute(min)
            .map(|t| NaiveDateTime { time: t, ..*self })
    }

    /// Makes a new `NaiveDateTime` with the second number changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    /// As with the [`second`](#method.second) method,
    /// the input range is restricted to 0 through 59.
    ///
    /// See also the
    /// [`NaiveTime::with_second`](./struct.NaiveTime.html#method.with_second) method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.with_second(17),
    ///            Some(NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 17, 789)));
    /// assert_eq!(dt.with_second(60), None);
    /// ~~~~
    #[inline]
    fn with_second(&self, sec: u32) -> Option<NaiveDateTime> {
        self.time
            .with_second(sec)
            .map(|t| NaiveDateTime { time: t, ..*self })
    }

    /// Makes a new `NaiveDateTime` with nanoseconds since the whole non-leap second changed.
    ///
    /// Returns `None` when the resulting `NaiveDateTime` would be invalid.
    /// As with the [`nanosecond`](#method.nanosecond) method,
    /// the input range can exceed 1,000,000,000 for leap seconds.
    ///
    /// See also the
    /// [`NaiveTime::with_nanosecond`](./struct.NaiveTime.html#method.with_nanosecond)
    /// method.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{NaiveDate, NaiveDateTime, Timelike};
    ///
    /// let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);
    /// assert_eq!(dt.with_nanosecond(333_333_333),
    ///            Some(NaiveDate::from_ymd(2015, 9, 8).and_hms_nano(12, 34, 56, 333_333_333)));
    /// assert_eq!(dt.with_nanosecond(1_333_333_333), // leap second
    ///            Some(NaiveDate::from_ymd(2015, 9, 8).and_hms_nano(12, 34, 56, 1_333_333_333)));
    /// assert_eq!(dt.with_nanosecond(2_000_000_000), None);
    /// ~~~~
    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<NaiveDateTime> {
        self.time
            .with_nanosecond(nano)
            .map(|t| NaiveDateTime { time: t, ..*self })
    }
}

/// An addition of `Duration` to `NaiveDateTime` yields another `NaiveDateTime`.
///
/// As a part of Chrono's [leap second handling](./struct.NaiveTime.html#leap-second-handling),
/// the addition assumes that **there is no leap second ever**,
/// except when the `NaiveDateTime` itself represents a leap second
/// in which case the assumption becomes that **there is exactly a single leap second ever**.
///
/// Panics on underflow or overflow.
/// Use [`NaiveDateTime::checked_add_signed`](#method.checked_add_signed) to detect that.
///
/// # Example
///
/// ~~~~
/// # extern crate chrono; fn main() {
/// use chrono::{Duration, NaiveDate};
///
/// let from_ymd = NaiveDate::from_ymd;
///
/// let d = from_ymd(2016, 7, 8);
/// let hms = |h, m, s| d.and_hms(h, m, s);
/// assert_eq!(hms(3, 5, 7) + Duration::zero(),             hms(3, 5, 7));
/// assert_eq!(hms(3, 5, 7) + Duration::seconds(1),         hms(3, 5, 8));
/// assert_eq!(hms(3, 5, 7) + Duration::seconds(-1),        hms(3, 5, 6));
/// assert_eq!(hms(3, 5, 7) + Duration::seconds(3600 + 60), hms(4, 6, 7));
/// assert_eq!(hms(3, 5, 7) + Duration::seconds(86_400),
///            from_ymd(2016, 7, 9).and_hms(3, 5, 7));
/// assert_eq!(hms(3, 5, 7) + Duration::days(365),
///            from_ymd(2017, 7, 8).and_hms(3, 5, 7));
///
/// let hmsm = |h, m, s, milli| d.and_hms_milli(h, m, s, milli);
/// assert_eq!(hmsm(3, 5, 7, 980) + Duration::milliseconds(450), hmsm(3, 5, 8, 430));
/// # }
/// ~~~~
///
/// Leap seconds are handled,
/// but the addition assumes that it is the only leap second happened.
///
/// ~~~~
/// # extern crate chrono; fn main() {
/// # use chrono::{Duration, NaiveDate};
/// # let from_ymd = NaiveDate::from_ymd;
/// # let hmsm = |h, m, s, milli| from_ymd(2016, 7, 8).and_hms_milli(h, m, s, milli);
/// let leap = hmsm(3, 5, 59, 1_300);
/// assert_eq!(leap + Duration::zero(),             hmsm(3, 5, 59, 1_300));
/// assert_eq!(leap + Duration::milliseconds(-500), hmsm(3, 5, 59, 800));
/// assert_eq!(leap + Duration::milliseconds(500),  hmsm(3, 5, 59, 1_800));
/// assert_eq!(leap + Duration::milliseconds(800),  hmsm(3, 6, 0, 100));
/// assert_eq!(leap + Duration::seconds(10),        hmsm(3, 6, 9, 300));
/// assert_eq!(leap + Duration::seconds(-10),       hmsm(3, 5, 50, 300));
/// assert_eq!(leap + Duration::days(1),
///            from_ymd(2016, 7, 9).and_hms_milli(3, 5, 59, 300));
/// # }
/// ~~~~
impl Add<OldDuration> for NaiveDateTime {
    type Output = NaiveDateTime;

    #[inline]
    fn add(self, rhs: OldDuration) -> NaiveDateTime {
        self.checked_add_signed(rhs)
            .expect("`NaiveDateTime + Duration` overflowed")
    }
}

/// The `Debug` output of the naive date and time `dt` is the same as
/// [`dt.format("%Y-%m-%dT%H:%M:%S%.f")`](../format/strftime/index.html).
///
/// The string printed can be readily parsed via the `parse` method on `str`.
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ~~~~
/// use chrono::NaiveDate;
///
/// let dt = NaiveDate::from_ymd(2016, 11, 15).and_hms(7, 39, 24);
/// assert_eq!(format!("{:?}", dt), "2016-11-15T07:39:24");
/// ~~~~
///
/// Leap seconds may also be used.
///
/// ~~~~
/// # use chrono::NaiveDate;
/// let dt = NaiveDate::from_ymd(2015, 6, 30).and_hms_milli(23, 59, 59, 1_500);
/// assert_eq!(format!("{:?}", dt), "2015-06-30T23:59:60.500");
/// ~~~~
impl fmt::Debug for NaiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}T{:?}", self.date, self.time)
    }
}

/// The `Display` output of the naive date and time `dt` is the same as
/// [`dt.format("%Y-%m-%d %H:%M:%S%.f")`](../format/strftime/index.html).
///
/// It should be noted that, for leap seconds not on the minute boundary,
/// it may print a representation not distinguishable from non-leap seconds.
/// This doesn't matter in practice, since such leap seconds never happened.
/// (By the time of the first leap second on 1972-06-30,
/// every time zone offset around the world has standardized to the 5-minute alignment.)
///
/// # Example
///
/// ~~~~
/// use chrono::NaiveDate;
///
/// let dt = NaiveDate::from_ymd(2016, 11, 15).and_hms(7, 39, 24);
/// assert_eq!(format!("{}", dt), "2016-11-15 07:39:24");
/// ~~~~
///
/// Leap seconds may also be used.
///
/// ~~~~
/// # use chrono::NaiveDate;
/// let dt = NaiveDate::from_ymd(2015, 6, 30).and_hms_milli(23, 59, 59, 1_500);
/// assert_eq!(format!("{}", dt), "2015-06-30 23:59:60.500");
/// ~~~~
impl fmt::Display for NaiveDateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
