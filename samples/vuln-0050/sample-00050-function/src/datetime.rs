// This is a part of Chrono.
// See README.md and LICENSE.txt for details.

//! ISO 8601 date and time with time zone.

use core::fmt;
use core::ops::Add;
use oldtime::Duration as OldDuration;

#[cfg(feature = "std")]
#[cfg(any(feature = "alloc", feature = "std", test))]
use crate::naive;
#[cfg(feature = "clock")]
use crate::offset::{Offset, TimeZone};
use crate::Date;
use naive::NaiveDateTime;

/// ISO 8601 combined date and time with time zone.
///
/// There are some constructors implemented here (the `from_*` methods), but
/// the general-purpose constructors are all via the methods on the
/// [`TimeZone`](./offset/trait.TimeZone.html) implementations.
#[derive(Clone)]
pub struct DateTime<Tz: TimeZone> {
    datetime: NaiveDateTime,
    offset: Tz::Offset,
}

impl<Tz: TimeZone> DateTime<Tz> {
    /// Makes a new `DateTime` with given *UTC* datetime and offset.
    /// The local datetime should be constructed via the `TimeZone` trait.
    ///
    /// # Example
    ///
    /// ~~~~
    /// use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    ///
    /// let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
    /// assert_eq!(Utc.timestamp(61, 0), dt);
    /// ~~~~
    //
    // note: this constructor is purposely not named to `new` to discourage the direct usage.
    #[inline]
    pub fn from_utc(datetime: NaiveDateTime, offset: Tz::Offset) -> DateTime<Tz> {
        DateTime {
            datetime: datetime,
            offset: offset,
        }
    }

    // /// Retrieves a date component.
    // #[inline]
    pub fn date(&self) -> Date<Tz> {
        Date::from_utc(self.naive_local().date(), self.offset.clone())
    }

    // /// Retrieves an associated offset from UTC.
    // #[inline]
    pub fn offset(&self) -> &Tz::Offset {
        &self.offset
    }

    /// Retrieves an associated time zone.
    #[inline]
    pub fn timezone(&self) -> Tz {
        TimeZone::from_offset(&self.offset)
    }

    /// Changes the associated time zone.
    /// This does not change the actual `DateTime` (but will change the string representation).
    #[inline]
    pub fn with_timezone<Tz2: TimeZone>(&self, tz: &Tz2) -> DateTime<Tz2> {
        tz.from_utc_datetime(&self.datetime)
    }

    /// Adds given `Duration` to the current date and time.
    ///
    /// Returns `None` when it will result in overflow.
    #[inline]
    pub fn checked_add_signed(self, rhs: OldDuration) -> Option<DateTime<Tz>> {
        let datetime = try_opt!(self.datetime.checked_add_signed(rhs));
        let tz = self.timezone();
        Some(tz.from_utc_datetime(&datetime))
    }

    /// Returns a view to the naive UTC datetime.
    #[inline]
    pub fn naive_utc(&self) -> NaiveDateTime {
        self.datetime
    }

    #[inline]
    pub fn naive_local(&self) -> NaiveDateTime {
        self.datetime + self.offset.fix()
    }
}

impl<Tz: TimeZone> fmt::Debug for DateTime<Tz> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.naive_local(), self.offset)
    }
}

impl<Tz: TimeZone> fmt::Display for DateTime<Tz>
where
    Tz::Offset: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.naive_local(), self.offset)
    }
}
